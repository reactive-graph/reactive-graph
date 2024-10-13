use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManPagesGenerationError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 Error: {0}")]
    Utf8(#[from] FromUtf8Error),
}
