// This example shows how to use the "app" helpers to get a window open and drawing with minimal code
// It's not as flexible as working with winit directly, but it's quick and simple

use skulpin::AppControl;
use skulpin::CoordinateSystemHelper;
use skulpin::InputState;
use skulpin::LogicalSize;
use skulpin::TimeState;
use skulpin::VirtualKeyCode;
use skulpin::{AppHandler, CoordinateSystem};
use std::ffi::CString;

use drawing_robot::bezier::{calc_point_iterator, MoveType, Point, PointIterator};
use std::collections::LinkedList;
use svgtypes::PathSegment;

fn points_to_draw() -> LinkedList<Point> {
    let start_point = Point { x: 0., y: 0. };
    let svg_string =
        "M10 80 C 40 10, 65 10, 95 80 S 150 150, 180 80 M10 280 Q 52.5 210, 95 280 T 180 280 T 250 280";
    let path_parser = svgtypes::PathParser::from(svg_string);

    let mut points: LinkedList<Point> = LinkedList::new();
    let mut current_point = start_point;
    let mut prev_segment: Option<PathSegment> = None;
    for token in path_parser {
        if let Ok(path_segment) = token {
            let point_iterator = calc_point_iterator(current_point, path_segment, prev_segment);
            if (point_iterator.move_type != MoveType::Fly) {
                for point in point_iterator {
                    points.push_back(point);
                }
                current_point = points.back().unwrap().clone();
            } else {
                current_point = point_iterator.last().unwrap();
            }
            prev_segment = Some(path_segment);
        }
    }

    points
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
        time_state: &TimeState,
        canvas: &mut skia_safe::Canvas,
        _coordinate_system_helper: &CoordinateSystemHelper,
    ) {
        // Generally would want to clear data every time we draw
        canvas.clear(skia_safe::Color::from_argb(0, 0, 0, 255));

        // Floating point value constantly moving between 0..1 to generate some movement
        let f = ((time_state.update_count() as f32 / 30.0).sin() + 1.0) / 2.0;

        // Make a color to draw with
        let mut paint = skia_safe::Paint::new(skia_safe::Color4f::new(1., 0., 0., 1.0), None);
        paint.set_anti_alias(true);
        paint.set_style(skia_safe::paint::Style::Stroke);
        paint.set_stroke_width(2.0);

        // Draw a line

        let points = points_to_draw();
        for point in points {
            canvas.draw_point(
                skia_safe::Point::new(point.x as f32, point.y as f32),
                &paint,
            );
        }
    }

    fn fatal_error(&mut self, error: &skulpin::AppError) {
        println!("{}", error);
    }
}
