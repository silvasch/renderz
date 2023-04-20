mod app;
pub use app::{App, AppBuilder};
pub mod builtin;
mod color;
pub use color::Color;
mod error;
pub use error::RenderzError;
pub mod renderer;
