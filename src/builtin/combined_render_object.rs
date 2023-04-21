use crate::prelude::*;

pub struct CombinedRenderObject {
    render_objects: Vec<Box<dyn RenderObject>>,
}

impl CombinedRenderObject {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> CombinedRenderObjectBuilder {
        CombinedRenderObjectBuilder::new()
    }
}

impl RenderObject for CombinedRenderObject {
    fn as_vertices(&self) -> (Vec<Vertex>, Option<Vec<u16>>) {
        let mut out_vertices: Vec<Vertex> = vec![];
        let mut out_indices: Vec<u16> = vec![];
        let mut index_offset: usize = 0;
        for render_object in &self.render_objects {
            let (vertices, indices) = render_object.as_vertices();
            let num_vertices = vertices.len();
            for vertex in vertices {
                out_vertices.push(vertex);
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
        (out_vertices, Some(out_indices))
    }
}

pub struct CombinedRenderObjectBuilder {
    render_objects: Vec<Box<dyn RenderObject>>,
}

impl CombinedRenderObjectBuilder {
    pub fn new() -> Self {
        Self {
            render_objects: vec![],
        }
    }

    pub fn with_render_object(mut self, render_object: Box<dyn RenderObject>) -> Self {
        self.render_objects.push(render_object);
        self
    }

    pub fn build(self) -> CombinedRenderObject {
        CombinedRenderObject {
            render_objects: self.render_objects,
        }
    }
}

impl Default for CombinedRenderObjectBuilder {
    fn default() -> Self {
        Self::new()
    }
}
