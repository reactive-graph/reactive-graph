use thiserror::Error;

use inexor_rgf_graph::prelude::*;

use crate::error::serde::DeserializationError;
use crate::error::serde::SerializationError;

#[derive(Debug, Error)]
pub enum FlowTypeRegistrationError {
    #[error("The flow type {0} already exists")]
    FlowTypeAlreadyExists(FlowTypeId),
    // OutboundComponentDoesNotExist(RelationTypeId, ComponentTypeId),
    // OutboundEntityTypeDoesNotExist(RelationTypeId, EntityTypeId),
    // InboundComponentDoesNotExist(RelationTypeId, ComponentTypeId),
    // InboundEntityTypeDoesNotExist(RelationTypeId, EntityTypeId),
}

#[derive(Debug, Error)]
pub enum FlowTypeCreationError {
    #[error("Failed to create flow type because registration failed: {0}")]
    RegistrationError(#[from] FlowTypeRegistrationError),
}

#[derive(Debug, Error)]
pub enum FlowTypeImportError {
    #[error("Failed to import flow type because reading failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to import flow type because format {0} is not supported!")]
    UnsupportedFormat(String),
    #[error("Failed to import flow type because deserialization failed: {0}")]
    Deserialization(#[from] DeserializationError),
    #[error("Failed to import flow type because registration failed: {0}")]
    RegistrationError(#[from] FlowTypeRegistrationError),
}

#[derive(Debug, Error)]
pub enum FlowTypeExportError {
    #[error("Failed to export non-existing flow type {0}")]
    FlowTypeNotFound(FlowTypeId),
    #[error("Failed to export flow type because write failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to export flow type because format {0} is not supported!")]
    UnsupportedFormat(String),
    #[error("Failed to export flow type because serialization failed: {0}")]
    Serialization(#[from] SerializationError),
}
