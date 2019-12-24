use svgtypes::PathSegment;
use drawing_robot::bezier::{Point, calc_rename_me};

fn main() {
    let point = Point{x: 10., y: 20.};
    let path_segment = PathSegment::MoveTo{abs: true, x: 20.0, y: 30.};

    let iter = calc_rename_me(point, path_segment);

    for p in iter {
        println!("x: {:?};\t\ty: {:?}", p.x, p.y);
    }
}