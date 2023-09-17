use crate::model::ComponentTypeId;
use crate::model::EntityTypeId;
use crate::model::RelationTypeId;

#[derive(Debug)]
pub enum RelationTypeRegistrationError {
    RelationTypeAlreadyExists(RelationTypeId),
    OutboundComponentDoesNotExist(RelationTypeId, ComponentTypeId),
    OutboundEntityTypeDoesNotExist(RelationTypeId, EntityTypeId),
    InboundComponentDoesNotExist(RelationTypeId, ComponentTypeId),
    InboundEntityTypeDoesNotExist(RelationTypeId, EntityTypeId),
}

#[derive(Debug)]
pub enum RelationTypeCreationError {
    RegistrationError(RelationTypeRegistrationError),
}

#[derive(Debug)]
pub enum RelationTypeImportError {
    Io(std::io::Error),
    Deserialize(serde_json::Error),
    RegistrationError(RelationTypeRegistrationError),
}

impl From<std::io::Error> for RelationTypeImportError {
    fn from(e: std::io::Error) -> Self {
        RelationTypeImportError::Io(e)
    }
}

impl From<serde_json::Error> for RelationTypeImportError {
    fn from(e: serde_json::Error) -> Self {
        RelationTypeImportError::Deserialize(e)
    }
}

#[derive(Debug)]
pub enum RelationTypeExportError {
    RelationTypeNotFound(RelationTypeId),
    Io(std::io::Error),
    Serialization(serde_json::Error),
}
