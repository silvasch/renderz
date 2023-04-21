use crate::prelude::*;

pub struct Quad {
    pub corners: [Position; 4],
    pub color: Color,
}

impl Quad {
    pub fn new(
        corner_one: Position,
        corner_two: Position,
        corner_three: Position,
        corner_four: Position,
        color: Color,
    ) -> Self {
        Self {
            corners: [corner_one, corner_two, corner_three, corner_four],
            color,
        }
    }
}

impl RenderObject for Quad {
    fn as_vertices(&self) -> (Vec<crate::Vertex>, Option<Vec<u16>>) {
        (
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
                Vertex {
                    position: self.corners[3],
                    color: self.color,
                },
            ],
            Some(vec![0, 1, 3, 1, 2, 3]),
        )
    }
}
