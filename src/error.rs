#[derive(Debug, thiserror::Error)]
pub enum RenderzError {
    #[error("unknown error: {0}")]
    UnknownError(String),
}
