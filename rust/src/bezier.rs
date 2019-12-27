use svgtypes::{PathCommand, PathSegment};

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

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct SupportPoint {
    path_command: PathCommand,
    point: Point,
}

pub struct PointIterator {
    time: BezierTick,
    calc_formula: Box<dyn Fn(f64) -> Point>,
    pub move_type: MoveType,
    support_point: Option<SupportPoint>, //support point is always in absolute
}

impl PointIterator {
    pub fn get_support_point(&self) -> Option<SupportPoint> {
        match &self.support_point {
            Some(supp_p) => Some(SupportPoint {
                path_command: supp_p.path_command,
                point: Point {
                    x: supp_p.point.x,
                    y: supp_p.point.y,
                },
            }),
            None => None,
        }
    }

    pub fn get_end_position(&self) -> Point {
        (self.calc_formula)(1.0)
    }
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
    Box::new(move |t: f64| {
        let x = start.x * (1. - t) + end.x * t;
        let y = start.y * (1. - t) + end.y * t;
        Point { x, y }
    })
}

fn square_curve(start: Point, p1: Point, end: Point) -> Box<dyn Fn(f64) -> Point> {
    Box::new(move |t: f64| {
        let diff = 1. - t;
        let square_t = t * t;
        let square_diff = diff * diff;
        let x = start.x * square_diff + p1.x * 2. * t * diff + end.x * square_t;
        let y = start.y * square_diff + p1.y * 2. * t * diff + end.y * square_t;
        Point { x, y }
    })
}

fn cubic_curve(start: Point, p1: Point, p2: Point, end: Point) -> Box<dyn Fn(f64) -> Point> {
    Box::new(move |t: f64| {
        let diff = 1. - t;
        let square_t = t * t;
        let cube_t = square_t * t;
        let square_diff = diff * diff;
        let cube_diff = square_diff * diff;
        let x = start.x * cube_diff
            + p1.x * 3. * t * square_diff
            + p2.x * 3. * square_t * diff
            + end.x * cube_t;
        let y = start.y * cube_diff
            + p1.y * 3. * t * square_diff
            + p2.y * 3. * square_t * diff
            + end.y * cube_t;
        Point { x, y }
    })
}

pub fn calc_point_iterator(
    current: Point,
    next_segment: PathSegment,
    prev_supp_point_opt: Option<SupportPoint>,
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
                support_point: None,
            }
        }
        PathSegment::LineTo { abs, x, y } => {
            let end_point = absolute_point_coord(&current, abs, x, y);
            let calc_formula = linear_curve(current, end_point);
            PointIterator {
                time,
                calc_formula,
                move_type: MoveType::Draw,
                support_point: None,
            }
        }
        PathSegment::HorizontalLineTo { abs, x } => {
            let end_point = absolute_point_coord(&current, abs, x, current.y);
            let calc_formula = linear_curve(current, end_point);
            PointIterator {
                time,
                calc_formula: Box::new(calc_formula),
                move_type: MoveType::Draw,
                support_point: None,
            }
        }
        PathSegment::VerticalLineTo { abs, y } => {
            let end_point = absolute_point_coord(&current, abs, current.x, y);
            let calc_formula = linear_curve(current, end_point);
            PointIterator {
                time,
                calc_formula: Box::new(calc_formula),
                move_type: MoveType::Draw,
                support_point: None,
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
            let support_point = Some(SupportPoint {
                path_command: next_segment.cmd(),
                point: Point { x: p2.x, y: p2.y },
            });
            let calc_formula = cubic_curve(current, p1, p2, end_point);
            PointIterator {
                time,
                calc_formula: Box::new(calc_formula),
                move_type: MoveType::Draw,
                support_point,
            }
        }
        PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
            let p1 = match prev_supp_point_opt {
                Some(prev_support_point) => {
                    if (prev_support_point.path_command == PathCommand::SmoothCurveTo
                        || prev_support_point.path_command == PathCommand::CurveTo)
                    {
                        let mirrored_x = current.x + current.x - prev_support_point.point.x;
                        let mirrored_y = current.y + current.y - prev_support_point.point.y;
                        Point {
                            x: mirrored_x,
                            y: mirrored_y,
                        }
                    } else {
                        Point { x: x2, y: y2 }
                    }
                }
                None => Point { x: x2, y: y2 },
            };
            let end_point = absolute_point_coord(&current, abs, x, y);
            let p2 = absolute_point_coord(&current, abs, x2, y2);
            let support_point = Some(SupportPoint {
                path_command: next_segment.cmd(),
                point: Point { x: p2.x, y: p2.y },
            });
            let calc_formula = cubic_curve(current, p1, p2, end_point);
            PointIterator {
                time,
                calc_formula: Box::new(calc_formula),
                move_type: MoveType::Draw,
                support_point,
            }
        }
        PathSegment::Quadratic { abs, x1, y1, x, y } => {
            let end_point = absolute_point_coord(&current, abs, x, y);
            let p1 = absolute_point_coord(&current, abs, x1, y1);
            let support_point = Some(SupportPoint {
                path_command: next_segment.cmd(),
                point: Point { x: p1.x, y: p1.y },
            });
            let calc_formula = square_curve(current, p1, end_point);
            PointIterator {
                time,
                calc_formula: Box::new(calc_formula),
                move_type: MoveType::Draw,
                support_point,
            }
        }
        PathSegment::SmoothQuadratic { abs, x, y } => {
            let p1 = match prev_supp_point_opt {
                Some(prev_support_point) => {
                    if (prev_support_point.path_command == PathCommand::SmoothQuadratic
                        || prev_support_point.path_command == PathCommand::Quadratic)
                    {
                        let mirrored_x = current.x + current.x - prev_support_point.point.x;
                        let mirrored_y = current.y + current.y - prev_support_point.point.y;
                        Point {
                            x: mirrored_x,
                            y: mirrored_y,
                        }
                    } else {
                        Point {
                            x: current.x,
                            y: current.y,
                        }
                    }
                }
                None => Point {
                    x: current.x,
                    y: current.y,
                },
            };
            let end_point = absolute_point_coord(&current, abs, x, y);
            let support_point = Some(SupportPoint {
                path_command: next_segment.cmd(),
                point: Point { x: p1.x, y: p1.y },
            });
            let calc_formula = square_curve(current, p1, end_point);
            PointIterator {
                time,
                calc_formula: Box::new(calc_formula),
                move_type: MoveType::Draw,
                support_point,
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
                support_point: None,
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
