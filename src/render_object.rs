use crate::Vertex;

pub trait RenderObject {
    #[allow(unused)]
    fn update(&mut self, delta_time: f32) {}
    fn as_vertices(&self) -> (Vec<Vertex>, Option<Vec<u16>>) {
        (vec![], None)
    }
}
