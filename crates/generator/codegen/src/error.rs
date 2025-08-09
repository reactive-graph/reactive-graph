use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodeGenerationError {
    #[error("Failed to write source code to {0}: {1}")]
    WriteError(PathBuf, std::io::Error),
    #[error("Failed to create source code folder {0}")]
    PathError(PathBuf),
}
