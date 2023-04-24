#![allow(clippy::disallowed_methods)]

use renderz::prelude::*;

fn main() {
    App::new()
        .with_initial_window_size((800, 600))
        .with_render_object(Box::new(Rectangle::new(
            Position::new(300.0, 250.0, 0.0),
            Size::new(200.0, 100.0, 0.0),
            Color::BLACK,
        )))
        .build()
        .unwrap()
        .run()
        .unwrap()
}
