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

    pub fn to_vertices(&self, screen_size: &winit::dpi::PhysicalSize<u32>) -> Vec<RenderingVertex> {
        let mut out: Vec<RenderingVertex> = vec![];
        for render_object in &self.render_objects {
            for vertex in render_object.as_vertices() {
                out.push(RenderingVertex {
                    position: vertex.position.as_screenspace_coords(screen_size),
                    color: vertex.color.as_slice(),
                });
            }
        }
        out
    }
}
