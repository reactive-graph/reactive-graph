use crate::error::reactive::entity::ReactiveEntityCreationError;
use uuid::Uuid;

#[derive(Debug)]
pub enum EntityInstanceImportError {
    Io(std::io::Error),
    Deserialization(serde_json::Error),
    EntityAlreadyExists(Uuid),
    ReactiveEntityCreationError(ReactiveEntityCreationError),
}

impl From<std::io::Error> for EntityInstanceImportError {
    fn from(e: std::io::Error) -> Self {
        EntityInstanceImportError::Io(e)
    }
}

impl From<serde_json::Error> for EntityInstanceImportError {
    fn from(e: serde_json::Error) -> Self {
        EntityInstanceImportError::Deserialization(e)
    }
}

#[derive(Debug)]
pub enum EntityInstanceExportError {
    EntityNotFound(Uuid),
    Io(String),
    Serialization(serde_json::Error),
}
