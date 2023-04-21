#[derive(Clone, Copy)]
pub struct Rotation {
    pub x_rot: f32,
    pub y_rot: f32,
    pub z_rot: f32,
}

impl Rotation {
    pub fn new(x_rot: f32, y_rot: f32, z_rot: f32) -> Self {
        Self {
            x_rot,
            y_rot,
            z_rot,
        }
    }
}
