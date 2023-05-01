use crate::prelude::*;

pub struct Circle {
    pub position: Position,
    pub resolution: u32,
    pub radius: f32,
    pub color: Color,
}

impl Circle {
    pub fn new(position: Position, resolution: u32, radius: f32, color: Color) -> Self {
        Self {
            position,
            resolution,
            radius,
            color,
        }
    }

    fn generate_vertices(&self) -> (Vec<Vertex>, Vec<u16>) {
        let mut vertices: Vec<Vertex> = vec![Vertex {
            position: self.position,
            color: self.color,
        }];

        let vertex_angle = 360.0 / self.resolution as f32;

        for triangle_num in 1..self.resolution {
            let rotation_angle = triangle_num as f32 * vertex_angle;

            let angle_distance: f32;
            let offset_x_prefix: i8;
            let offset_y_prefix: i8;
            if rotation_angle <= 90.0 {
                angle_distance = rotation_angle;
                (offset_x_prefix, offset_y_prefix) = (1, -1);
            } else if rotation_angle <= 180.0 {
                angle_distance = rotation_angle - 90.0;
                (offset_x_prefix, offset_y_prefix) = (1, 1);
            } else if rotation_angle <= 270.0 {
                angle_distance = rotation_angle - 180.0;
                (offset_x_prefix, offset_y_prefix) = (-1, 1);
            } else {
                angle_distance = rotation_angle - 270.0;
                (offset_x_prefix, offset_y_prefix) = (-1, -1);
            };

            // Calculate offsets
            let offset_x = self.radius * angle_distance.sin() * offset_x_prefix as f32;
            let offset_y = self.radius * angle_distance.cos() * offset_y_prefix as f32;

            vertices.push(Vertex {
                position: Position::new(
                    self.position.x + offset_x,
                    self.position.y + offset_y,
                    self.position.z,
                ),
                color: self.color,
            });
        }

        let mut indices: Vec<u16> = vec![];
        for triangle_num in 0..self.resolution {
            indices.append(&mut vec![
                0,
                triangle_num as u16 + 1,
                triangle_num as u16 + 2,
            ]);
        }

        (vertices, indices)
    }
}

impl RenderObject for Circle {
    fn as_vertices(&self) -> (Vec<Vertex>, Option<Vec<u16>>) {
        let (vertices, indices) = self.generate_vertices();
        (vertices, Some(indices))
    }
}
