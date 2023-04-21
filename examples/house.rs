#![allow(clippy::disallowed_methods)]

use renderz::prelude::*;

pub fn house() -> CombinedRenderObject {
    CombinedRenderObject::new()
        .with_render_object(Box::new(Quad::new(
            Position::new(400.0, 300.0, 0.0),
            Position::new(440.0, 300.0, 0.0),
            Position::new(440.0, 260.0, 0.0),
            Position::new(400.0, 260.0, 0.0),
            Color::BLACK,
        )))
        .with_render_object(Box::new(Triangle::new(
            Position::new(400.0, 260.0, 0.0),
            Position::new(440.0, 260.0, 0.0),
            Position::new(420.0, 240.0, 0.0),
            Color::BLACK,
        )))
        .build()
}

fn main() {
    App::new()
        .with_render_object(Box::new(house()))
        .build()
        .unwrap()
        .run()
        .unwrap()
}
