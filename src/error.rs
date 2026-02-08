use thiserror::Error;

#[derive(Error, Debug)]
pub enum LeptonError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("LEB128 read error")]
    Leb128Read(#[from] leb128::read::Error),

    #[error("Zstd error: {0}")]
    Zstd(std::io::Error), // Remove #[from] to avoid conflict with Io(#[from] std::io::Error)

    #[error("Invalid magic bytes")]
    InvalidMagic,
}

pub type LeptonResult<T> = Result<T, LeptonError>;
