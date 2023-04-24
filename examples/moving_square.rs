#![allow(clippy::disallowed_methods)]

use renderz::prelude::*;

struct MovingSquare {
    rect: Rectangle,
    dir: i8,
}

impl MovingSquare {
    pub fn new() -> Self {
        Self {
            rect: Rectangle::new(
                Position::new(350.0, 250.0, 0.0),
                Size::new(100.0, 100.0, 0.0),
                Color::BLACK,
            ),
            dir: 1,
        }
    }
}

impl RenderObject for MovingSquare {
    fn update(&mut self, delta_time: f32) {
        if self.rect.position.x > 550.0 {
            self.dir = -1;
        } else if self.rect.position.x < 150.0 {
            self.dir = 1;
        }

        self.rect.position.x += delta_time / 5.0 * self.dir as f32;
    }

    fn as_vertices(&self) -> (Vec<Vertex>, Option<Vec<u16>>) {
        self.rect.as_vertices()
    }
}

fn main() {
    App::new()
        .with_initial_window_size((800, 600))
        .with_render_object(Box::new(MovingSquare::new()))
        .build()
        .unwrap()
        .run()
        .unwrap()
}
