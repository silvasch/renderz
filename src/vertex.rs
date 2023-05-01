use crate::{rendering_vertex::RenderingVertex, transform::Position, Color};

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Position,
    pub color: Color,
}

impl Vertex {
    pub(crate) fn as_rendering_vertex(
        &self,
        screen_size: &winit::dpi::PhysicalSize<u32>,
    ) -> RenderingVertex {
        RenderingVertex {
            position: self.position.as_screen_coords(screen_size),
            color: self.color.as_slice(),
        }
    }
}
