#[derive(Debug)]
pub enum HotDeployError {
    NoDynamicLinkLibrary,
    InvalidInstallPath,
    MoveError,
}
