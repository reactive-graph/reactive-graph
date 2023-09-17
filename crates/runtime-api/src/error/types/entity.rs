use crate::model::EntityTypeId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EntityTypeRegistrationError {
    #[error("Failed to register entity type {0} because it is already registered!")]
    EntityTypeAlreadyExists(EntityTypeId),
}

#[derive(Debug, Error)]
pub enum EntityTypeCreationError {
    #[error("Failed to create entity type: {0}")]
    RegistrationError(EntityTypeRegistrationError),
}

#[derive(Debug, Error)]
pub enum EntityTypeImportError {
    #[error("Failed to read entity type during import: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to deserialize entity type during import: {0}")]
    Deserialization(#[from] serde_json::Error),
    #[error("Failed to register entity type during import: {0}")]
    RegistrationError(EntityTypeRegistrationError),
}

#[derive(Debug, Error)]
pub enum EntityTypeExportError {
    #[error("The entity type {0} doesn't exist!")]
    EntityTypeNotFound(EntityTypeId),
    #[error("Failed to write entity type during export: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to serialize entity type during export: {0}")]
    Serialization(#[from] serde_json::Error),
}
