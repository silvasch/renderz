use crate::RenderingVertex;

use super::render_object::RenderObject;

pub struct RenderObjectsManager {
    render_objects: Vec<Box<dyn RenderObject>>,
}

impl RenderObjectsManager {
    pub fn new(render_objects: Vec<Box<dyn RenderObject>>) -> Self {
        Self { render_objects }
    }

    pub fn update(&mut self) {
        for render_object in &mut self.render_objects {
            render_object.update();
        }
    }

    pub fn to_vertices(
        &self,
        window_size: &winit::dpi::PhysicalSize<u32>,
    ) -> (Vec<RenderingVertex>, Vec<u16>) {
        let mut out_vertices: Vec<RenderingVertex> = vec![];
        let mut out_indices: Vec<u16> = vec![];
        let mut index_offset: usize = 0;
        for render_object in &self.render_objects {
            let (vertices, indices) = render_object.as_vertices();
            let num_vertices = vertices.len();
            for vertex in vertices {
                out_vertices.push(vertex.as_rendering_vertex(window_size));
            }

            match indices {
                Some(indices) => {
                    for index in indices {
                        out_indices.push(index + index_offset as u16);
                    }
                }
                None => {
                    for i in 0..num_vertices {
                        out_indices.push((i + index_offset) as u16);
                    }
                }
            }

            index_offset += num_vertices;
        }
        (out_vertices, out_indices)
    }
}
