use thiserror::Error;

use inexor_rgf_graph::prelude::*;

use crate::error::types::serde::DeserializationError;
use crate::error::types::serde::SerializationError;

#[derive(Debug, Error)]
pub enum ComponentRegistrationError {
    #[error("Failed to register component {0} because it already exists!")]
    ComponentAlreadyExists(ComponentTypeId),
}

#[derive(Debug, Error)]
pub enum ComponentCreationError {
    #[error("Failed to create component because registration failed: {0}")]
    RegistrationError(#[from] ComponentRegistrationError),
}

#[derive(Debug, Error)]
pub enum ComponentImportError {
    #[error("Failed to import component because reading failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to import component because format {0} is not supported!")]
    UnsupportedFormat(String),
    #[error("Failed to import component because deserialization failed: {0}")]
    Deserialization(#[from] DeserializationError),
    #[error("Failed to import component because registration failed: {0}")]
    RegistrationError(#[from] ComponentRegistrationError),
}

#[derive(Debug, Error)]
pub enum ComponentExportError {
    #[error("Failed to export non existent component {0}!")]
    ComponentNotFound(ComponentTypeId),
    #[error("Failed to export component because writing failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to export component because format {0} is not supported!")]
    UnsupportedFormat(String),
    #[error("Failed to export component because serialization failed: {0}")]
    Serialization(#[from] SerializationError),
}
