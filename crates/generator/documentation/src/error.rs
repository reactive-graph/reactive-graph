use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DocumentationGenerationError {
    #[error("Failed to write documentation to {0}: {1}")]
    WriteError(PathBuf, std::io::Error),
    #[error("Failed to create documentation folder {0}")]
    PathError(PathBuf),
}
