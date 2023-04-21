#[derive(Clone, Copy)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn as_screen_coords(&self, window_size: &winit::dpi::PhysicalSize<u32>) -> [f32; 3] {
        [
            self.x / window_size.width as f32 * 2.0 - 1.0,
            (self.y / window_size.height as f32 * 2.0 - 1.0) * -1.0,
            self.z,
        ]
    }

    pub fn x(&self) -> &f32 {
        &self.x
    }

    pub fn y(&self) -> &f32 {
        &self.y
    }

    pub fn z(&self) -> &f32 {
        &self.z
    }
}
