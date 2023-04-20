use crate::renderer::RenderObject;

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
    fn update(&mut self) {
        println!("Updated '{}'", self.name);
    }
}
