use crate::model::ComponentTypeId;

#[derive(Debug)]
pub enum ComponentRegistrationError {
    ComponentAlreadyExists(ComponentTypeId),
}

#[derive(Debug)]
pub enum ComponentCreationError {
    RegistrationError(ComponentRegistrationError),
}

#[derive(Debug)]
pub enum ComponentImportError {
    Io(std::io::Error),
    Deserialization(serde_json::Error),
    RegistrationError(ComponentRegistrationError),
}

impl From<std::io::Error> for ComponentImportError {
    fn from(e: std::io::Error) -> Self {
        ComponentImportError::Io(e)
    }
}

impl From<serde_json::Error> for ComponentImportError {
    fn from(e: serde_json::Error) -> Self {
        ComponentImportError::Deserialization(e)
    }
}

#[derive(Debug)]
pub enum ComponentExportError {
    ComponentNotFound(ComponentTypeId),
    Io(std::io::Error),
    Serialization(serde_json::Error),
}
