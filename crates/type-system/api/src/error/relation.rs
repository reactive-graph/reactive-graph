use thiserror::Error;

use reactive_graph_graph::prelude::*;

use crate::error::serde::DeserializationError;
use crate::error::serde::SerializationError;

#[derive(Debug, Error)]
pub enum RelationTypeRegistrationError {
    #[error("Failed to register relation type {0} because it already exists!")]
    RelationTypeAlreadyExists(RelationTypeId),
    #[error("Failed to register relation type {0} because outbound component {1} does not exist!")]
    OutboundComponentDoesNotExist(RelationTypeId, ComponentTypeId),
    #[error("Failed to register relation type {0} because outbound entity type {1} does not exist!")]
    OutboundEntityTypeDoesNotExist(RelationTypeId, EntityTypeId),
    #[error("Failed to register relation type {0} because inbound component {1} does not exist!")]
    InboundComponentDoesNotExist(RelationTypeId, ComponentTypeId),
    #[error("Failed to register relation type {0} because inbound entity type {1} does not exist!")]
    InboundEntityTypeDoesNotExist(RelationTypeId, EntityTypeId),
}

#[derive(Debug, Error)]
pub enum RelationTypeCreationError {
    #[error("Failed to create relation type because registration failed: {0}")]
    RegistrationError(#[from] RelationTypeRegistrationError),
}

#[derive(Debug, Error)]
pub enum RelationTypeImportError {
    #[error("Failed to import relation type because reading failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to import relation type because format {0} is not supported!")]
    UnsupportedFormat(String),
    #[error("Failed to import relation type because deserialization failed: {0}")]
    Deserialization(#[from] DeserializationError),
    #[error("Failed to import relation type because registration failed: {0}")]
    RegistrationError(#[from] RelationTypeRegistrationError),
}

#[derive(Debug, Error)]
pub enum RelationTypeExportError {
    #[error("Failed to export non existent relation type {0}!")]
    RelationTypeNotFound(RelationTypeId),
    #[error("Failed to export relation type because writing failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to export relation type because format {0} is not supported!")]
    UnsupportedFormat(String),
    #[error("Failed to export relation type because serialization failed: {0}")]
    Serialization(#[from] SerializationError),
}
