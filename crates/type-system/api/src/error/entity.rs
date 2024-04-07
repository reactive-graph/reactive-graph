use thiserror::Error;

use reactive_graph_graph::prelude::*;

use crate::error::serde::DeserializationError;
use crate::error::serde::SerializationError;

#[derive(Debug, Error)]
pub enum EntityTypeRegistrationError {
    #[error("Failed to register entity type {0} because it is already registered!")]
    EntityTypeAlreadyExists(EntityTypeId),
}

#[derive(Debug, Error)]
pub enum EntityTypeCreationError {
    #[error("Failed to create entity type: {0}")]
    RegistrationError(#[from] EntityTypeRegistrationError),
}

#[derive(Debug, Error)]
pub enum EntityTypeImportError {
    #[error("Failed to import entity type because reading failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to import entity type because format {0} is not supported!")]
    UnsupportedFormat(String),
    #[error("Failed to import entity type because deserialization failed: {0}")]
    Deserialization(#[from] DeserializationError),
    #[error("Failed to import entity type because registration failed: {0}")]
    RegistrationError(#[from] EntityTypeRegistrationError),
}

#[derive(Debug, Error)]
pub enum EntityTypeExportError {
    #[error("Failed to export non existent entity type {0}!")]
    EntityTypeNotFound(EntityTypeId),
    #[error("Failed to export entity type because writing failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to export entity type because format {0} is not supported!")]
    UnsupportedFormat(String),
    #[error("Failed to export entity type because serialization failed: {0}")]
    Serialization(#[from] SerializationError),
}
