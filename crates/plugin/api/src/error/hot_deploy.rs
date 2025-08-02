// TODO: derive Error
#[derive(Debug)]
pub enum HotDeployError {
    NoDynamicLinkLibrary,
    InvalidInstallPath,
    MoveError,
    ArchiveError, // TODO: Nested error
}
