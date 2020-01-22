use svgtypes::{PathCommand, PathSegment};

use super::math::*;
use super::tick_timer::TickTimer;
use core::ops::{Mul, Add, Sub, Div};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point {x, y}
    }

    pub const ZERO: Point = Point {x: 0., y: 0.};
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Add<f64> for Point {
    type Output = Point;

    fn add(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Sub<f64> for Point {
    type Output = Point;

    fn sub(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x - rhs,
            y: self.y - rhs
        }
    }
}

pub enum LineTo {
    Fly(Point),
    Draw(Point),
    Erase(Point),
}

impl LineTo {
    fn new(point: Point, move_type: MoveType) -> Self {
        match move_type {
            MoveType::Fly => LineTo::Fly(point),
            MoveType::Draw => LineTo::Draw(point),
            MoveType::Erase => LineTo::Erase(point),
        }
    }
}

pub fn points_from_path_segments<'a>(
    path_segments: impl Iterator<Item = PathSegment> + 'a,
) -> Box<dyn Iterator<Item = LineTo> + 'a> {
    let mut current_point = Point::ZERO;
    let mut prev_support_point_opt: Option<SupportPoint> = None;
    let mut path_start_point = Point::ZERO;
    let mut path_start_point_initialized = false;

    Box::new(path_segments.flat_map(move |path_segment| {
        let point_iterator = calc_point_iterator(
            current_point,
            path_segment,
            prev_support_point_opt,
            path_start_point,
        );
        prev_support_point_opt = point_iterator.get_support_point();
        current_point = point_iterator.get_end_position();

        if !path_start_point_initialized && path_segment.cmd() != PathCommand::ClosePath {
            path_start_point_initialized = true;
            path_start_point = current_point;
        } else if path_segment.cmd() == PathCommand::ClosePath {
            path_start_point_initialized = false;
        }

        let move_type = point_iterator.move_type();
        point_iterator.map(move |point| LineTo::new(point, move_type))
    }))
}

// === private members ===

#[derive(PartialEq, Copy, Clone)]
enum MoveType {
    Fly,
    Draw,
    Erase,
}

#[derive(Debug, Copy, Clone)]
struct SupportPoint {
    path_command: PathCommand,
    point: Point,
}

trait PointIterator: Iterator<Item = Point> {
    fn get_support_point(&self) -> Option<SupportPoint>; //support point is always in absolute
    fn get_end_position(&self) -> Point;
    fn move_type(&self) -> MoveType;
}

struct EmptyPointIterator {
    end: Point,
}

impl Iterator for EmptyPointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl PointIterator for EmptyPointIterator {
    fn get_support_point(&self) -> Option<SupportPoint> {
        None
    }

    fn get_end_position(&self) -> Point {
        self.end
    }

    fn move_type(&self) -> MoveType {
        MoveType::Fly
    }
}

struct LinePointIterator {
    end: Point,
    move_type: MoveType,
    done: bool,
    support_point: Option<SupportPoint>,
}

impl LinePointIterator {
    fn new(end: Point, move_type: MoveType) -> Self {
        LinePointIterator {
            end,
            move_type,
            done: false,
            support_point: None,
        }
    }

    fn as_fake_curve(
        end: Point,
        move_type: MoveType,
        support_point: Option<SupportPoint>,
    ) -> Self {
        LinePointIterator {
            end,
            move_type,
            done: false,
            support_point,
        }
    }
}

impl Iterator for LinePointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = true;
            Some(self.end)
        }
    }
}

impl PointIterator for LinePointIterator {
    fn get_support_point(&self) -> Option<SupportPoint> {
        self.support_point
    }

    fn get_end_position(&self) -> Point {
        self.end
    }

    fn move_type(&self) -> MoveType {
        self.move_type
    }
}

struct CurvePointIterator<F: Fn(f64) -> Point> {
    time: TickTimer,
    calc_formula: F,
    move_type: MoveType,
    support_point: Option<SupportPoint>,
}

impl<F: Fn(f64) -> Point> Iterator for CurvePointIterator<F> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.time.next() {
            Some(time) => Some((self.calc_formula)(time)),
            None => None,
        }
    }
}

impl<F: Fn(f64) -> Point> PointIterator for CurvePointIterator<F> {
    fn get_support_point(&self) -> Option<SupportPoint> {
        self.support_point
    }

    fn get_end_position(&self) -> Point {
        (self.calc_formula)(1.0)
    }

    fn move_type(&self) -> MoveType {
        self.move_type
    }
}

struct EllipsePointIterator<F: Fn(f64) -> Point> {
    time: TickTimer,
    calc_formula: F,
    end: Point,
}

impl<F: Fn(f64) -> Point> Iterator for EllipsePointIterator<F> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.time.next() {
            Some(time) => Some((self.calc_formula)(time)),
            None => None,
        }
    }
}

impl<F: Fn(f64) -> Point> PointIterator for EllipsePointIterator<F> {
    fn get_support_point(&self) -> Option<SupportPoint> {
        None
    }

    fn get_end_position(&self) -> Point {
        self.end
    }

    fn move_type(&self) -> MoveType {
        MoveType::Draw
    }
}

fn calc_point_iterator(
    current: Point,
    next_segment: PathSegment,
    prev_support_point_opt: Option<SupportPoint>,
    path_start_point: Point, //want that to implement ClosePath
) -> Box<dyn PointIterator> {
    match next_segment {
        PathSegment::MoveTo { abs, x, y } => Box::new(move_to(current, abs, x, y)),
        PathSegment::LineTo { abs, x, y } => Box::new(line_to(current, abs, x, y)),
        PathSegment::HorizontalLineTo { abs, x } => {
            let miss_coord = if abs { current.y } else { 0. };
            Box::new(line_to(current, abs, x, miss_coord))
        }
        PathSegment::VerticalLineTo { abs, y } => {
            let miss_coord = if abs { current.x } else { 0. };
            Box::new(line_to(current, abs, miss_coord, y))
        }
        PathSegment::CurveTo {
            abs,
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        } => cubic_curve_to(current, abs, x1, y1, x2, y2, x, y, next_segment),
        PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => smooth_cubic_curve_to(
            current,
            abs,
            x2,
            y2,
            x,
            y,
            prev_support_point_opt,
            next_segment,
        ),
        PathSegment::Quadratic { abs, x1, y1, x, y } => {
            quadratic_curve_to(current, abs, x1, y1, x, y, next_segment)
        }
        PathSegment::SmoothQuadratic { abs, x, y } => {
            smooth_quadratic_curve_to(current, abs, x, y, prev_support_point_opt, next_segment)
        }
        PathSegment::EllipticalArc {
            abs,
            rx,
            ry,
            x_axis_rotation,
            large_arc,
            sweep,
            x,
            y,
        } => ellipse_curve_to(
            current,
            abs,
            rx,
            ry,
            x_axis_rotation,
            large_arc,
            sweep,
            x,
            y,
        ),
        PathSegment::ClosePath { abs: _ } => Box::new(line_to(
            current,
            true,
            path_start_point.x,
            path_start_point.y,
        )),
    }
}

fn move_to(current: Point, abs: bool, x: f64, y: f64) -> LinePointIterator {
    let end_point = absolute_point_coord(current, abs, x, y);
    LinePointIterator::new(end_point, MoveType::Fly)
}

fn line_to(current: Point, abs: bool, x: f64, y: f64) -> LinePointIterator {
    let end_point = absolute_point_coord(current, abs, x, y);
    LinePointIterator::new(end_point, MoveType::Draw)
}

fn cubic_curve_to(
    current: Point,
    abs: bool,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x: f64,
    y: f64,
    next_segment: PathSegment,
) -> Box<dyn PointIterator> {
    let time: TickTimer = Default::default();
    let p1 = absolute_point_coord(current, abs, x1, y1);
    let p2 = absolute_point_coord(current, abs, x2, y2);
    let end_point = absolute_point_coord(current, abs, x, y);
    let support_point = Some(SupportPoint {
        path_command: next_segment.cmd(),
        point: p2,
    });

    let p1_on_lane = is_point_on_lane(current, end_point, &p1);
    let p2_on_lane = is_point_on_lane(current, end_point, &p2);

    if p1_on_lane && p2_on_lane {
        Box::new(LinePointIterator::as_fake_curve(
            end_point,
            MoveType::Draw,
            support_point,
        ))
    } else {
        let calc_formula = cubic_curve(current, p1, p2, end_point);
        Box::new(CurvePointIterator {
            time,
            calc_formula,
            move_type: MoveType::Draw,
            support_point,
        })
    }
}

fn smooth_cubic_curve_to(
    current: Point,
    abs: bool,
    x2: f64,
    y2: f64,
    x: f64,
    y: f64,
    prev_support_point_opt: Option<SupportPoint>,
    next_segment: PathSegment,
) -> Box<dyn PointIterator> {
    let p1 = mirrored_point(current, abs, prev_support_point_opt, CurveType::Cubic);
    cubic_curve_to(current, abs, p1.x, p1.y, x2, y2, x, y, next_segment)
}

fn quadratic_curve_to(
    current: Point,
    abs: bool,
    x1: f64,
    y1: f64,
    x: f64,
    y: f64,
    next_segment: PathSegment,
) -> Box<dyn PointIterator> {
    let time: TickTimer = Default::default();
    let p1 = absolute_point_coord(current, abs, x1, y1);
    let end_point = absolute_point_coord(current, abs, x, y);
    let support_point = Some(SupportPoint {
        path_command: next_segment.cmd(),
        point: Point { x: p1.x, y: p1.y },
    });

    let p1_on_lane = is_point_on_lane(current, end_point, &p1);
    if p1_on_lane {
        Box::new(LinePointIterator::as_fake_curve(
            end_point,
            MoveType::Draw,
            support_point,
        ))
    } else {
        let calc_formula = square_curve(current, p1, end_point);
        Box::new(CurvePointIterator {
            time,
            calc_formula,
            move_type: MoveType::Draw,
            support_point,
        })
    }
}

fn smooth_quadratic_curve_to(
    current: Point,
    abs: bool,
    x: f64,
    y: f64,
    prev_support_point_opt: Option<SupportPoint>,
    next_segment: PathSegment,
) -> Box<dyn PointIterator> {
    let p1 = mirrored_point(current, abs, prev_support_point_opt, CurveType::Quadratic);
    quadratic_curve_to(current, abs, p1.x, p1.y, x, y, next_segment)
}

fn ellipse_curve_to(
    current: Point,
    abs: bool,
    rx: f64,
    ry: f64,
    x_axis_rotation: f64,
    large_arc: bool,
    sweep: bool,
    end_x: f64,
    end_y: f64,
) -> Box<dyn PointIterator> {
    let time: TickTimer = Default::default();

    let end_point = absolute_point_coord(current, abs, end_x, end_y);

    // If the endpoints are identical, then this is equivalent to omitting the elliptical arc segment entirely.
    if current == end_point {
        return Box::new(EmptyPointIterator {
            end: end_point,
        });
    }

    // If rx = 0 or ry = 0 then this arc is treated as a straight line segment joining the endpoints.
    if rx == 0. || ry == 0. {
        return Box::new(line_to(current, abs, end_x, end_y));
    }

    let (start_angle, sweep_angle, rx_abs, ry_abs, x_rad_rotation, center_x, center_y) =
        ellipse_support_calc(
            current,
            rx,
            ry,
            x_axis_rotation,
            large_arc,
            sweep,
            end_point.x,
            end_point.y,
        );

    let calc_formula = ellipse_curve(
        start_angle,
        sweep_angle,
        rx_abs,
        ry_abs,
        x_rad_rotation,
        center_x,
        center_y,
    );
    Box::new(EllipsePointIterator {
        time,
        calc_formula,
        end: end_point
    })
}

fn absolute_point_coord(start: Point, abs: bool, x: f64, y: f64) -> Point {
    match abs {
        true => Point { x, y },
        false => Point{ x, y } + start,
    }
}

enum CurveType {
    Cubic,
    Quadratic,
}

fn path_command_condition(prev_support_point: &SupportPoint, curve_type: CurveType) -> bool {
    match curve_type {
        CurveType::Cubic => {
            prev_support_point.path_command == PathCommand::SmoothCurveTo
                || prev_support_point.path_command == PathCommand::CurveTo
        }

        CurveType::Quadratic => {
            prev_support_point.path_command == PathCommand::SmoothQuadratic
                || prev_support_point.path_command == PathCommand::Quadratic
        }
    }
}

fn mirrored_point(
    current: Point,
    abs: bool,
    prev_support_point_opt: Option<SupportPoint>,
    curve_type: CurveType,
) -> Point {
    let mut mirrored_point = match prev_support_point_opt {
        Some(ref prev_support_point) if path_command_condition(prev_support_point, curve_type) => {
            current - prev_support_point.point
        }
        _ => Point::ZERO,
    };

    if abs {
        mirrored_point = mirrored_point + current;
    }

    mirrored_point
}
