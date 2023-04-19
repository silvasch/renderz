use winit::window::Window;

pub struct Renderer {
    window: Window,
}

impl Renderer {
    pub fn new(window: Window) -> Self {
        Self { window }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
