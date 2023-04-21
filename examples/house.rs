#![allow(clippy::disallowed_methods)]

use renderz::prelude::*;

fn house() -> CombinedRenderObject {
    CombinedRenderObject::new()
        .with_render_object(Box::new(Quad::new(
            Position::new(450.0, 250.0, 0.0),
            Position::new(350.0, 250.0, 0.0),
            Position::new(350.0, 350.0, 0.0),
            Position::new(450.0, 350.0, 0.0),
            Color::BLACK,
        )))
        .with_render_object(Box::new(Triangle::new(
            Position::new(350.0, 250.0, 0.0),
            Position::new(450.0, 250.0, 0.0),
            Position::new(400.0, 180.0, 0.0),
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
