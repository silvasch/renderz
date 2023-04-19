#[derive(Debug, thiserror::Error)]
pub enum RenderzError {
    #[error("program is out of memory")]
    OutOfMemory,

    #[error("the swap chain has been lost and needs to be recreated")]
    WgpuSurfaceLost,
    #[error("the swap chain must be updated")]
    WgpuSurfaceOutdated,
    #[error("timed out while trying to aquire the next frame")]
    WgpuSurfaceTimeout,
    #[error("wgpu could not create a surface")]
    WgpuSurfaceCreationError,
    #[error("wgpu could not create an adapter")]
    WgpuAdapterCreationError,
    #[error("wgpu could not create a device")]
    WgpuDeviceCreationError,

    #[error("winit could not create a window")]
    WinitWindowCreationError,

    #[error("unknown error: {0}")]
    UnknownError(String),
}

impl From<wgpu::SurfaceError> for RenderzError {
    fn from(value: wgpu::SurfaceError) -> Self {
        match value {
            wgpu::SurfaceError::OutOfMemory => RenderzError::OutOfMemory,
            wgpu::SurfaceError::Lost => RenderzError::WgpuSurfaceLost,
            wgpu::SurfaceError::Outdated => RenderzError::WgpuSurfaceOutdated,
            wgpu::SurfaceError::Timeout => RenderzError::WgpuSurfaceTimeout,
        }
    }
}
