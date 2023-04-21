use crate::Vertex;

pub trait RenderObject {
    fn update(&mut self) {}
    fn to_vertices(&self) -> Vec<Vertex> {
        vec![]
    }
}
