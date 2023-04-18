use crate::{RenderzError, renderer::Renderer};

pub struct App {
    #[allow(unused)]
    renderer: Renderer,
}

impl App {
    pub fn new() -> AppBuilder {
        AppBuilder::new()
    }

    pub fn run(self) -> Result<(), RenderzError> {
        Ok(())
    }
}

pub struct AppBuilder {}

impl AppBuilder {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub fn build(self) -> App {
        let renderer = Renderer::new();

        App {
            renderer,
        }
    }
}
