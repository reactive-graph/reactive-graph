use crate::EntityTypeId;
use crate::ExtensionTypeId;
use crate::NamespacedTypeParseError;
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

#[derive(Debug, Error)]
pub enum ExtensionJsonSchemaError {
    #[error("The extension {0} has no schema entity type")]
    NoSchemaEntityType(ExtensionTypeId),
    #[error("The extension {0} has a schema entity type {1} which does not exist")]
    SchemaEntityTypeDoesNotExist(ExtensionTypeId, EntityTypeId),
    #[error("The given entity type {2} does not match the schema entity type {1} of the extension {0}")]
    SchemaEntityTypeDoesNotMatch(ExtensionTypeId, EntityTypeId, EntityTypeId),
    #[error("Failed to modify the json schema")]
    SchemaModificationError,
}

#[derive(Debug, Error)]
pub enum InvalidExtensionError {
    #[error("The fully qualified namespace of the extension is invalid: {0}")]
    InvalidExtension(NamespacedTypeParseError),
    #[error("The fully qualified namespace of the entity type is invalid: {0}")]
    InvalidEntityType(NamespacedTypeParseError),
}
