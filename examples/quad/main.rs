#![allow(clippy::disallowed_methods)]

use renderz::prelude::*;

fn main() {
    App::new()
        .is_resizable(true)
        .with_initial_size((800, 600))
        .with_background_color(Color::from_rgba(0, 255, 255, 0.5))
        .with_render_object(Box::new(builtin::Quad::new(
            Position::new(330.0, 280.0, 0.0),
            Position::new(330.0, 320.0, 0.0),
            Position::new(370.0, 320.0, 0.0),
            Position::new(370.0, 280.0, 0.0),
            Color::RED,
        )))
        .with_render_object(Box::new(builtin::Quad::new(
            Position::new(430.0, 280.0, 0.0),
            Position::new(430.0, 320.0, 0.0),
            Position::new(470.0, 320.0, 0.0),
            Position::new(470.0, 280.0, 0.0),
            Color::GREEN,
        )))
        .build()
        .unwrap()
        .run()
        .unwrap();
}
