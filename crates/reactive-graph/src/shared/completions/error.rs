use clap_complete::Shell;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstallShellCompletionError {
    #[error("Failed to get xdg base directory: The shell {0} is not supported")]
    UnsupportedShell(Shell),
    #[error("Failed to get xdg base directory: {0}")]
    BaseDirectories(#[from] xdg::BaseDirectoriesError),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}
