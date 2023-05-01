#![allow(clippy::disallowed_methods)]

use renderz::prelude::*;

pub struct DebugCircle {
    circle: Circle,
}

impl DebugCircle {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            circle: Circle::new(Position::new(400.0, 300.0, 0.0), 100, 80.0, Color::BLACK),
        }
    }
}

impl RenderObject for DebugCircle {
    fn update(&mut self, _delta_time: f32) {}

    fn as_vertices(&self) -> (Vec<Vertex>, Option<Vec<u16>>) {
        self.circle.as_vertices()
    }
}

fn main() {
    App::new()
        .with_initial_window_size((800, 600))
        .with_render_object(Box::new(DebugCircle::new()))
        .build()
        .unwrap()
        .run()
        .unwrap()
}
