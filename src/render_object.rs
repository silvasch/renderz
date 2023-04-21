use crate::Vertex;

pub trait RenderObject {
    fn update(&mut self) {}
    fn as_vertices(&self) -> (Vec<Vertex>, Option<Vec<u16>>) {
        (vec![], None)
    }
}
