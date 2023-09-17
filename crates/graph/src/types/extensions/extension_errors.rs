use crate::ExtensionTypeId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AddExtensionError {
    #[error("The extension {0} already exists")]
    ExtensionAlreadyExist(ExtensionTypeId),
}

#[derive(Debug, Error)]
pub enum UpdateExtensionError {
    #[error("The extension {0} does not exist")]
    ExtensionDoesNotExist(ExtensionTypeId),
}

#[derive(Debug, Error)]
pub enum RemoveExtensionError {
    #[error("The extension {0} does not exist")]
    ExtensionDoesNotExist(ExtensionTypeId),
}
