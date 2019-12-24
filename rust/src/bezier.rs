use svgtypes::PathSegment;
use std::ops::Mul;
use std::ops::Add;

struct BezierTick {
    pub time: f64
}

impl BezierTick {
    //    pub const TICK_PERIOD: f64 = 0.001; //todo: uncomment production value
    const TICK_PERIOD: f64 = 0.1;

    fn new() -> BezierTick {
        BezierTick { time: 0.0 }
    }
}

impl Iterator for BezierTick {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        println!("current time: {:?}", self.time);
        if (self.time > 1.0) {
            None
        } else {
            let current_value = self.time;
            self.time += BezierTick::TICK_PERIOD;
            Some(current_value)
        }
    }
}

pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {x: self.x * rhs, y: self.y * rhs}
    }
}

impl Add for Point{
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

struct PointIterator {
    time: BezierTick,
    start: Point,
    end: Point,
    calc_formula: Box<dyn Fn(f64, &Point, &Point) -> Point>,
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.time.next() {
            Some(time) => Some((self.calc_formula)(time, &self.start, &self.end)),
            None => None
        }
    }
}

const LINEAR_CURVE: (fn(f64, &Point, &Point) -> Point) = |t: f64, start: &Point, end: &Point| start * (1. - t) + end * t;

pub fn calc_rename_me(current: Point, path_segment: PathSegment) -> impl Iterator<Item = Point> {
    let time = BezierTick::new();

    match path_segment {
        PathSegment::MoveTo{abs, x, y} =>
            {
                let end_point = calc_end_point(&current, abs, x, y);
                PointIterator {time, start: current, end: end_point, calc_formula: Box::new(LINEAR_CURVE) }
            },
//  todo: implement other Bezier curves
//        PathSegment::LineTo{abs, x, y} => (),
//        PathSegment::HorizontalLineTo{abs, x} => (),
//        PathSegment::VerticalLineTo{abs, y} => (),
//        PathSegment::CurveTo{abs, x1, y1, x2, y2, x, y} => (),
//        PathSegment::SmoothCurveTo{abs, x2, y2, x, y} => (),
//        PathSegment::Quadratic{abs, x1, y1, x, y} => (),
//        PathSegment::SmoothQuadratic{abs, x, y} => (),
//        PathSegment::EllipticalArc{abs, rx, ry, x_axis_rotation, large_arc, sweep, x, y} => (),
//        PathSegment::ClosePath{abs} => ()
        _ => {
            //todo: remove me
            let end_point = calc_end_point(&current, true, 20., 33.);
            PointIterator {time, start: current, end: end_point, calc_formula: Box::new(LINEAR_CURVE) }
        }
    }

}

fn calc_end_point(start: &Point, abs: bool, x: f64, y: f64) -> Point {
    match abs {
        true => Point {x, y},
        false => Point {x: x + start.x, y: y + start.y}
    }
}