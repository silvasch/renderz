use crate::prelude::*;

pub struct DebugObject {
    name: String,
}

impl DebugObject {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl RenderObject for DebugObject {
    fn update(&mut self, delta_time: f32) {
        println!("Updated '{}'\n\tDelta: {}", self.name, delta_time);
    }
}
