use super::svg_curve::Point;

const PI: f64 = 3.14159265358979323846264338327950288_f64;

pub fn square_curve(
    start_x: f64,
    start_y: f64,
    p1: Point,
    end: Point,
) -> Box<dyn Fn(f64) -> Point> {
    Box::new(move |t: f64| {
        let diff = 1. - t;
        let square_t = t * t;
        let square_diff = diff * diff;
        let x = start_x * square_diff + p1.x * 2. * t * diff + end.x * square_t;
        let y = start_y * square_diff + p1.y * 2. * t * diff + end.y * square_t;
        Point { x, y }
    })
}

pub fn cubic_curve(
    start_x: f64,
    start_y: f64,
    p1: Point,
    p2: Point,
    end: Point,
) -> Box<dyn Fn(f64) -> Point> {
    Box::new(move |t: f64| {
        let diff = 1. - t;
        let square_t = t * t;
        let cube_t = square_t * t;
        let square_diff = diff * diff;
        let cube_diff = square_diff * diff;
        let x = start_x * cube_diff
            + p1.x * 3. * t * square_diff
            + p2.x * 3. * square_t * diff
            + end.x * cube_t;
        let y = start_y * cube_diff
            + p1.y * 3. * t * square_diff
            + p2.y * 3. * square_t * diff
            + end.y * cube_t;
        Point { x, y }
    })
}

pub fn ellipse_curve(
    start_angle: f64,
    sweep_angle: f64,
    rx_abs: f64,
    ry_abs: f64,
    x_rad_rotation: f64,
    center_x: f64,
    center_y: f64,
) -> Box<dyn Fn(f64) -> Point> {
    Box::new(move |t: f64| {
        let angle = start_angle + sweep_angle * t;
        let ellipse_component_x = rx_abs * angle.cos();
        let ellipse_component_y = ry_abs * angle.sin();

        let point_x = x_rad_rotation.cos() * ellipse_component_x
            - x_rad_rotation.sin() * ellipse_component_y
            + center_x;
        let point_y = x_rad_rotation.sin() * ellipse_component_x
            + x_rad_rotation.cos() * ellipse_component_y
            + center_y;

        Point {
            x: point_x,
            y: point_y,
        }
    })
}

pub fn ellipse_support_calc(
    current: &Point,
    rx: f64,
    ry: f64,
    x_axis_rotation: f64,
    large_arc: bool,
    sweep: bool,
    end_x: f64,
    end_y: f64,
) -> (f64, f64, f64, f64, f64, f64, f64) {
    //calculations from: https://github.com/MadLittleMods/svg-curve-lib/

    let start_x = current.x;
    let start_y = current.y;

    let mut rx_abs = rx.abs();
    let mut ry_abs = ry.abs();
    let x_axis_rotation_mod_360 = x_axis_rotation % 360.0;
    let x_rad_rotation: f64 = x_axis_rotation_mod_360 * PI / 180.0;

    let dx = (start_x - end_x) / 2.;
    let dy = (start_y - end_y) / 2.;

    // Step #1: Compute transformedPoint
    let dx_rotated = x_rad_rotation.cos() * dx + x_rad_rotation.sin() * dy;
    let dy_rotated = -x_rad_rotation.sin() * dx + x_rad_rotation.cos() * dy;

    let radii_check = sqr(dx_rotated) / sqr(rx_abs) + sqr(dy_rotated) / sqr(ry_abs);
    if radii_check > 1.0 {
        rx_abs = radii_check.sqrt() * rx_abs;
        ry_abs = radii_check.sqrt() * ry_abs;
    }

    // Step #2: Compute transformedCenter
    let center_square_numerator =
        sqr(rx_abs) * sqr(ry_abs) - sqr(rx_abs) * sqr(dy_rotated) - sqr(ry_abs) * sqr(dx_rotated);
    let center_square_root_denom = sqr(rx_abs) * sqr(dy_rotated) + sqr(ry_abs) * sqr(dx_rotated);
    let mut center_radicand = center_square_numerator / center_square_root_denom;
    if center_radicand < 0. {
        center_radicand = 0.
    };

    let center_coef = if large_arc != sweep {
        1. * center_radicand.sqrt()
    } else {
        -1. * center_radicand.sqrt()
    };
    let center_x_rotated = center_coef * (rx_abs * dy_rotated / ry_abs);
    let center_y_rotated = center_coef * (-ry_abs * dx_rotated / rx_abs);

    // Step #3: Compute center
    let center_x = x_rad_rotation.cos() * center_x_rotated
        - x_rad_rotation.sin() * center_y_rotated
        + ((start_x + end_x) / 2.);
    let center_y = x_rad_rotation.sin() * center_x_rotated
        + x_rad_rotation.cos() * center_y_rotated
        + ((start_y + end_y) / 2.);

    // Step #4: Compute start/sweep angles
    let start_vector_x = (dx_rotated - center_x_rotated) / rx_abs;
    let start_vector_y = (dy_rotated - center_y_rotated) / ry_abs;
    let start_angle = angle_between(1., 0., start_vector_x, start_vector_y);

    let end_vector_x = (-dx_rotated - center_x_rotated) / rx_abs;
    let end_vector_y = (-dy_rotated - center_y_rotated) / ry_abs;
    let mut sweep_angle = angle_between(start_vector_x, start_vector_y, end_vector_x, end_vector_y);
    if !sweep && sweep_angle > 0. {
        sweep_angle -= 2. * PI;
    } else if sweep && sweep_angle < 0. {
        sweep_angle += 2. * PI;
    }
    sweep_angle = sweep_angle % (2. * PI);

    (
        start_angle,
        sweep_angle,
        rx_abs,
        ry_abs,
        x_rad_rotation,
        center_x,
        center_y,
    )
}

pub fn sqr(x: f64) -> f64 {
    x * x
}

pub fn angle_between(start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> f64 {
    let p = start_x * end_x + start_y * end_y;
    let n = ((sqr(start_x) + sqr(start_y)) * (sqr(end_x) + sqr(end_y))).sqrt();
    let sign = if start_x * end_y - start_y * end_x < 0. {
        -1.
    } else {
        1.
    };
    let angle = sign * (p / n).acos();
    return angle;
}

const EPSILON: f64 = 0.05;
pub fn is_point_on_lane(lane_start: &Point, lane_end: &Point, p: &Point) -> bool {
    let vector_x = lane_end.x - lane_start.x;
    let vector_y = lane_end.y - lane_start.y;

    let left_part = if vector_x == 0. {
        0.
    } else {
        (p.x - lane_start.x) / vector_x
    };
    let right_part = if vector_y == 0. {
        0.
    } else {
        (p.y - lane_start.y) / vector_y
    };

    let is_on_lane = left_part - right_part;
    is_on_lane.abs() < EPSILON
}
