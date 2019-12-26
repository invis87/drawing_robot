use std::ops::Add;
use std::ops::Mul;
use svgtypes::PathSegment;

#[derive(PartialEq)]
pub enum MoveType {
    Fly,
    Draw,
    Erase,
}

struct BezierTick {
    pub time: f64,
}

impl BezierTick {
    const TICK_PERIOD: f64 = 0.001; //todo: number of ticks should be calculated based on curve length

    fn new() -> BezierTick {
        BezierTick { time: 0.0 }
    }
}

impl Iterator for BezierTick {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.time > 1.0) {
            None
        } else {
            let current_value = self.time;
            self.time += BezierTick::TICK_PERIOD;
            Some(current_value)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct PointIterator {
    time: BezierTick,
    calc_formula: Box<dyn Fn(f64) -> Point>,
    pub move_type: MoveType,
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.time.next() {
            Some(time) => Some((self.calc_formula)(time)),
            None => None,
        }
    }
}

fn linear_curve(start: Point, end: Point) -> Box<dyn Fn(f64) -> Point> {
    Box::new(move |t: f64| start * (1. - t) + end * t)
}

fn square_curve(start: Point, p1: Point, end: Point) -> Box<dyn Fn(f64) -> Point> {
    Box::new(move |t: f64| {
        let diff = 1. - t;
        let square_t = t * t;
        let square_diff = diff * diff;
        start * square_diff + p1 * 2. * t * diff + end * square_t
    })
}

fn cubic_curve(start: Point, p1: Point, p2: Point, end: Point) -> Box<dyn Fn(f64) -> Point> {
    Box::new(move |t: f64| {
        let diff = 1. - t;
        let square_t = t * t;
        let cube_t = square_t * t;
        let square_diff = diff * diff;
        let cube_diff = square_diff * diff;
        start * cube_diff + p1 * 3. * t * square_diff + p2 * 3. * square_t * diff + end * cube_t
    })
}

pub fn calc_point_iterator(
    current: Point,
    next_segment: PathSegment,
    prev_segment_opt: Option<PathSegment>,
) -> PointIterator {
    let time = BezierTick::new();

    match next_segment {
        PathSegment::MoveTo { abs, x, y } => {
            let end_point = absolute_point_coord(&current, abs, x, y);
            let calc_formula = linear_curve(current, end_point);
            PointIterator {
                time,
                calc_formula,
                move_type: MoveType::Fly,
            }
        }
        PathSegment::LineTo { abs, x, y } => {
            let end_point = absolute_point_coord(&current, abs, x, y);
            let calc_formula = linear_curve(current, end_point);
            PointIterator {
                time,
                calc_formula,
                move_type: MoveType::Draw,
            }
        }
        PathSegment::HorizontalLineTo { abs, x } => {
            let end_point = absolute_point_coord(&current, abs, x, current.y);
            let calc_formula = linear_curve(current, end_point);
            PointIterator {
                time,
                calc_formula: Box::new(calc_formula),
                move_type: MoveType::Draw,
            }
        }
        PathSegment::VerticalLineTo { abs, y } => {
            let end_point = absolute_point_coord(&current, abs, current.x, y);
            let calc_formula = linear_curve(current, end_point);
            PointIterator {
                time,
                calc_formula: Box::new(calc_formula),
                move_type: MoveType::Draw,
            }
        }
        PathSegment::CurveTo {
            abs,
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        } => {
            let end_point = absolute_point_coord(&current, abs, x, y);
            let p1 = absolute_point_coord(&current, abs, x1, y1);
            let p2 = absolute_point_coord(&current, abs, x2, y2);
            let calc_formula = cubic_curve(current, p1, p2, end_point);
            PointIterator {
                time,
                calc_formula: Box::new(calc_formula),
                move_type: MoveType::Draw,
            }
        }
        PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
            if let Some(prev_segment) = prev_segment_opt {}
            let p1 = match prev_segment_opt {
                Some(PathSegment::CurveTo {
                    abs,
                    x1: _,
                    y1: _,
                    x2,
                    y2,
                    x: _,
                    y: _,
                }) => {
                    if (abs) {
                        let mirrored_x = current.x + current.x - x2;
                        let mirrored_y = current.y + current.y - y2;
                        Point {
                            x: mirrored_x,
                            y: mirrored_y,
                        }
                    } else {
                        Point {
                            x: current.x + x2,
                            y: current.y - y2,
                        }
                    }
                }
                Some(PathSegment::SmoothCurveTo { abs, x2, y2, x, y }) => {
                    if (abs) {
                        let mirrored_x = current.x + current.x - x2;
                        let mirrored_y = current.y + current.y - y2;
                        Point {
                            x: mirrored_x,
                            y: mirrored_y,
                        }
                    } else {
                        Point {
                            x: current.x + x2,
                            y: current.y - y2,
                        }
                    }
                }
                _ => Point { x: x2, y: y2 },
            };
            let end_point = absolute_point_coord(&current, abs, x, y);
            let p2 = absolute_point_coord(&current, abs, x2, y2);
            let calc_formula = cubic_curve(current, p1, p2, end_point);
            PointIterator {
                time,
                calc_formula: Box::new(calc_formula),
                move_type: MoveType::Draw,
            }
        }
        PathSegment::Quadratic { abs, x1, y1, x, y } => {
            let end_point = absolute_point_coord(&current, abs, x, y);
            let p1 = absolute_point_coord(&current, abs, x1, y1);
            let calc_formula = square_curve(current, p1, end_point);
            PointIterator {
                time,
                calc_formula: Box::new(calc_formula),
                move_type: MoveType::Draw,
            }
        }
        PathSegment::SmoothQuadratic { abs, x, y } => {
            let p1 = match prev_segment_opt {
                Some(PathSegment::Quadratic { abs, x1, y1, x, y }) => {
                    if (abs) {
                        let mirrored_x = current.x + current.x - x1;
                        let mirrored_y = current.y + current.y - y1;
                        Point {
                            x: mirrored_x,
                            y: mirrored_y,
                        }
                    } else {
                        Point {
                            x: current.x + x1,
                            y: current.y - y1,
                        }
                    }
                }
                Some(PathSegment::SmoothQuadratic { abs, x, y }) => {
                    if (abs) {
                        unimplemented!();
                    } else {
                        unimplemented!();
                    }
                }
                _ => Point { x, y },
            };
            let end_point = absolute_point_coord(&current, abs, x, y);
            let calc_formula = square_curve(current, p1, end_point);
            PointIterator {
                time,
                calc_formula: Box::new(calc_formula),
                move_type: MoveType::Draw,
            }
        }
        //        PathSegment::EllipticalArc{abs, rx, ry, x_axis_rotation, large_arc, sweep, x, y} => (),
        //        PathSegment::ClosePath{abs} => ()
        _ => {
            //todo: remove me
            let end_point = absolute_point_coord(&current, true, 20., 33.);
            let calc_formula = linear_curve(current, end_point);
            PointIterator {
                time,
                calc_formula,
                move_type: MoveType::Fly,
            }
        }
    }
}

fn absolute_point_coord(start: &Point, abs: bool, x: f64, y: f64) -> Point {
    match abs {
        true => Point { x, y },
        false => Point {
            x: x + start.x,
            y: y + start.y,
        },
    }
}
