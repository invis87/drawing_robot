#![allow(unused_parens)]

use skulpin::AppControl;
use skulpin::CoordinateSystemHelper;
use skulpin::InputState;
use skulpin::LogicalSize;
use skulpin::TimeState;
use skulpin::VirtualKeyCode;
use skulpin::{AppHandler, CoordinateSystem};
use std::ffi::CString;

use drawing_robot::svg_curve::{calc_point_iterator, MoveType, Point, PointIterator, SupportPoint};
use std::collections::LinkedList;

fn points_to_draw() -> LinkedList<Box<dyn PointIterator>> {
    let start_point = Point { x: 0., y: 0. };
    let svg_string =
//    "M 10,30 A 20,20 0,0,1 50,30 A 20,20 0,0,1 90,30 Q 90,60 50,90 Q 10,60 10,30"; // heart
//    "M10 80 C 40 10, 65 10, 95 80 S 150 150, 180 80 M10 280 Q 52.5 210, 95 280 T 180 280 T 250 280 M10 380 H 100 T 250 380 M10 480 Q 50 100, 95 480 S 150 550, 130 450";
//    "M 110 215 A 36 60 0 0 0 150.71 170.29 M 110 215 A 36 60 0 0 1 150.71 170.29 M 110 215 A 36 60 0 1 0 150.71 170.29 M 110 215 A 36 60 0 1 1 150.71 170.29"; // 4 curves that creates 2 ellipses

      "M10 80 C 40 10, 65 10, 95 80 S 150 150, 180 80 M10 280 Q 52.5 210, 95 280 T 180 280 T 250 280 M10 380 H 100 T 250 380 M10 480 Q 50 100, 95 480 S 150 550, 130 450 M10 680 A 30 50 0 0 1 62 627 L 80 630 A 3 5 -45 0 1 415 650";
    //        "M10 80 l 100 200 L 210 80";
    let path_parser = svgtypes::PathParser::from(svg_string);

    let mut point_iterators: LinkedList<Box<dyn PointIterator>> = LinkedList::new();
    let mut current_point = start_point;
    let mut prev_support_point_opt: Option<SupportPoint> = None;
    for token in path_parser {
        if let Ok(path_segment) = token {
            let point_iterator =
                calc_point_iterator(current_point, path_segment, prev_support_point_opt);
            prev_support_point_opt = point_iterator.get_support_point();
            current_point = point_iterator.get_end_position();
            point_iterators.push_back(point_iterator);
        }
    }

    point_iterators
}

fn main() {
    let example_app = ExampleApp::new();

    // Set up the coordinate system to be fixed at 900x600, and use this as the default window size
    // This means the drawing code can be written as though the window is always 900x600. The
    // output will be automatically scaled so that it's always visible.
    let logical_size = LogicalSize::new(1000.0, 1000.0);
    let visible_range = skulpin::skia_safe::Rect {
        left: 0.0,
        right: logical_size.width as f32,
        top: 0.0,
        bottom: logical_size.height as f32,
    };
    let scale_to_fit = skulpin::skia_safe::matrix::ScaleToFit::Center;

    skulpin::AppBuilder::new()
        .app_name(CString::new("Debug drawing").unwrap())
        .use_vulkan_debug_layer(true)
        .logical_size(logical_size)
        .coordinate_system(CoordinateSystem::VisibleRange(visible_range, scale_to_fit))
        .run(example_app);
}

struct ExampleApp {}

impl ExampleApp {
    pub fn new() -> Self {
        ExampleApp {}
    }
}

impl AppHandler for ExampleApp {
    fn update(
        &mut self,
        app_control: &mut AppControl,
        input_state: &InputState,
        _time_state: &TimeState,
    ) {
        if input_state.is_key_down(VirtualKeyCode::Escape) {
            app_control.enqueue_terminate_process();
        }
    }

    fn draw(
        &mut self,
        _app_control: &AppControl,
        _input_state: &InputState,
        _time_state: &TimeState,
        canvas: &mut skia_safe::Canvas,
        _coordinate_system_helper: &CoordinateSystemHelper,
    ) {
        // Generally would want to clear data every time we draw
        canvas.clear(skia_safe::Color::from_argb(0, 0, 0, 255));

        // Make a color to draw with
        let mut paint = skia_safe::Paint::new(skia_safe::Color4f::new(1., 0., 0., 1.0), None);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Stroke);
        paint.set_stroke_width(2.0);

        // Draw SVG
        let points_iterator = points_to_draw();
        let mut prev_init = false;
        let mut prev_point: Point = Point { x: 0.0, y: 0.0 };
        for points in points_iterator {
            let current_move_type = *points.move_type();
            for point in points {
                if !prev_init {
                    prev_init = true;
                    prev_point = point;
                } else {
                    if current_move_type != MoveType::Fly {
                        canvas.draw_line(
                            skia_safe::Point::new(prev_point.x as f32, prev_point.y as f32),
                            skia_safe::Point::new(point.x as f32, point.y as f32),
                            &paint,
                        );
                        prev_point = point;
                    } else {
                        prev_init = false;
                    }
                }
            }
        }
    }

    fn fatal_error(&mut self, error: &skulpin::AppError) {
        println!("{}", error);
    }
}
