use crate::Vertex;

pub trait RenderObject {
    fn update(&mut self) {}
    fn as_vertices(&self) -> Vec<Vertex> {
        vec![]
    }
}
