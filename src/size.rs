#[derive(Clone, Copy)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> &f32 {
        &self.width
    }

    pub fn height(&self) -> &f32 {
        &self.height
    }
}
