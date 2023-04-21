#![allow(clippy::disallowed_methods)]

use renderz::prelude::*;

fn main() {
    App::new()
        .with_initial_window_size((800, 600))
        .with_render_object(Box::new(Triangle::new(
            Position::new(0.0, 0.0, 0.0),
            Position::new(0.0, 100.0, 0.0),
            Position::new(100.0, 0.0, 0.0),
            Color::BLACK,
        )))
        .with_render_object(Box::new(Triangle::new(
            Position::new(800.0, 600.0, 0.0),
            Position::new(800.0, 500.0, 0.0),
            Position::new(700.0, 600.0, 0.0),
            Color::BLACK,
        )))
        .with_render_object(Box::new(Triangle::new(
            Position::new(0.0, 600.0, 0.0),
            Position::new(100.0, 600.0, 0.0),
            Position::new(0.0, 500.0, 0.0),
            Color::BLACK,
        )))
        .with_render_object(Box::new(Triangle::new(
            Position::new(800.0, 0.0, 0.0),
            Position::new(700.0, 0.0, 0.0),
            Position::new(800.0, 100.0, 0.0),
            Color::BLACK,
        )))
        .build()
        .unwrap()
        .run()
        .unwrap();
}
