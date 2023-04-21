#![allow(clippy::disallowed_methods)]

use renderz::prelude::*;

fn main() {
    App::new()
        .is_resizable(true)
        .with_initial_size((800, 600))
        .with_background_color(Color::from_rgba(0, 255, 255, 0.5))
        .with_render_object(Box::new(Triangle::new(
            Position::new(0.0, 0.0, 0.0),
            Position::new(0.0, 40.0, 0.0),
            Position::new(40.0, 0.0, 0.0),
            Color::RED,
        )))
        .with_render_object(Box::new(Triangle::new(
            Position::new(800.0, 600.0, 0.0),
            Position::new(800.0, 560.0, 0.0),
            Position::new(760.0, 600.0, 0.0),
            Color::RED,
        )))
        .with_render_object(Box::new(Triangle::new(
            Position::new(0.0, 600.0, 0.0),
            Position::new(40.0, 600.0, 0.0),
            Position::new(0.0, 560.0, 0.0),
            Color::RED,
        )))
        .with_render_object(Box::new(Triangle::new(
            Position::new(800.0, 0.0, 0.0),
            Position::new(760.0, 0.0, 0.0),
            Position::new(800.0, 40.0, 0.0),
            Color::RED,
        )))
        .build()
        .unwrap()
        .run()
        .unwrap();
}
