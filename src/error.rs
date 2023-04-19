#[derive(Debug, thiserror::Error)]
pub enum RenderzError {
    #[error("winit could not create a window")]
    WinitWindowCreationError,

    #[error("unknown error: {0}")]
    UnknownError(String),
}
