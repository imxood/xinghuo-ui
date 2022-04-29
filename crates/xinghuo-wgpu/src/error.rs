use thiserror::Error;

#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum BackendError {
    #[error("{0}")]
    Error(String),

    #[error("{0}")]
    InvalidTextureId(String),

    #[error("{0}")]
    Internal(String),

    #[error("SurfaceError")]
    SurfaceError(#[from] wgpu::SurfaceError),

    #[error("未知错误, 请联系开发人员.")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, BackendError>;

impl From<String> for BackendError {
    fn from(e: String) -> Self {
        BackendError::Error(e)
    }
}
