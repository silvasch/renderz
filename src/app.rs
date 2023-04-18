use crate::RenderzError;

pub struct App {}

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
        App {}
    }
}
