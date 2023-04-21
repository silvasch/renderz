use crate::prelude::*;

pub struct Triangle {
    pub corners: [Position; 3],
    pub color: Color,
}

impl Triangle {
    pub fn new(
        corner_one: Position,
        corner_two: Position,
        corner_three: Position,
        color: Color,
    ) -> Self {
        Self {
            corners: [corner_one, corner_two, corner_three],
            color,
        }
    }
}

impl RenderObject for Triangle {
    fn as_vertices(&self) -> Vec<crate::Vertex> {
        vec![
            Vertex {
                position: self.corners[0],
                color: self.color,
            },
            Vertex {
                position: self.corners[1],
                color: self.color,
            },
            Vertex {
                position: self.corners[2],
                color: self.color,
            },
        ]
    }
}
