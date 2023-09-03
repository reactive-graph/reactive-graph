use crate::model::EntityTypeId;

#[derive(Debug)]
pub enum EntityTypeRegistrationError {
    EntityTypeAlreadyExists(EntityTypeId),
}

#[derive(Debug)]
pub enum EntityTypeCreationError {
    RegistrationError(EntityTypeRegistrationError),
}

#[derive(Debug)]
pub enum EntityTypeImportError {
    Io(std::io::Error),
    Deserialization(serde_json::Error),
    RegistrationError(EntityTypeRegistrationError),
}


impl From<std::io::Error> for EntityTypeImportError {
    fn from(e: std::io::Error) -> Self {
        EntityTypeImportError::Io(e)
    }
}

impl From<serde_json::Error> for EntityTypeImportError {
    fn from(e: serde_json::Error) -> Self {
        EntityTypeImportError::Deserialization(e)
    }
}

#[derive(Debug)]
pub enum EntityTypeExportError {
    EntityTypeNotFound(EntityTypeId),
    Io(std::io::Error),
    Serialization(serde_json::Error),
}
