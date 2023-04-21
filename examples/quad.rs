#![allow(clippy::disallowed_methods)]

use renderz::prelude::*;

fn main() {
    App::new()
        .with_initial_window_size((800, 600))
        .with_render_object(Box::new(Quad::new(
            Position::new(450.0, 250.0, 0.0),
            Position::new(350.0, 250.0, 0.0),
            Position::new(350.0, 350.0, 0.0),
            Position::new(450.0, 350.0, 0.0),
            Color::BLACK,
        )))
        .build()
        .unwrap()
        .run()
        .unwrap();
}
