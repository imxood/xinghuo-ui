use thiserror::Error;

// #[non_exhaustive]
#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("{0}")]
    Any(String),

    #[error("未知错误, 请联系开发人员.")]
    Unknown,

    #[error("尺寸参数错误")]
    SizeFormatError,
}

pub type Result<T> = std::result::Result<T, Error>;
