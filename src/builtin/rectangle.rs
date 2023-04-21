use crate::prelude::*;

pub struct Rectangle {
    pub position: Position,
    pub size: Size,
    pub color: Color,
}

impl Rectangle {
    pub fn new(position: Position, size: Size, color: Color) -> Self {
        Self {
            position,
            size,
            color,
        }
    }
}

impl RenderObject for Rectangle {
    fn as_vertices(&self) -> (Vec<Vertex>, Option<Vec<u16>>) {
        (
            vec![
                Vertex {
                    position: self.position,
                    color: self.color,
                },
                Vertex {
                    position: Position::new(
                        *self.position.x(),
                        self.position.y() + self.size.height(),
                        *self.position.z(),
                    ),
                    color: self.color,
                },
                Vertex {
                    position: Position::new(
                        self.position.x() + self.size.width(),
                        self.position.y() + self.size.height(),
                        *self.position.z(),
                    ),
                    color: self.color,
                },
                Vertex {
                    position: Position::new(
                        self.position.x() + self.size.width(),
                        *self.position.y(),
                        *self.position.z(),
                    ),
                    color: self.color,
                },
            ],
            Some(vec![0, 1, 3, 1, 2, 3]),
        )
    }
}
