#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
}
