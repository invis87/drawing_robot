use drawing_robot::svg_curve::{calc_point_iterator, Point};
use svgtypes::PathSegment;

mod svg;

use svg::svg_curve::calc_point_iterator;

fn main() {
    let point = Point { x: 10., y: 20. };
    let next_segment = PathSegment::MoveTo {
        abs: true,
        x: 20.0,
        y: 30.,
    };

    let iter = calc_point_iterator(point, next_segment, None);

    for p in iter {
        println!("x: {:?};\t\ty: {:?}", p.x, p.y);
    }
}
