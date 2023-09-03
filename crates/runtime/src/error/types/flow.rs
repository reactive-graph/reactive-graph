use inexor_rgf_core_model::FlowTypeId;

#[derive(Debug)]
pub enum FlowTypeRegistrationError {
    FlowTypeAlreadyExists(FlowTypeId),
    // OutboundComponentDoesNotExist(RelationTypeId, ComponentTypeId),
    // OutboundEntityTypeDoesNotExist(RelationTypeId, EntityTypeId),
    // InboundComponentDoesNotExist(RelationTypeId, ComponentTypeId),
    // InboundEntityTypeDoesNotExist(RelationTypeId, EntityTypeId),
}

#[derive(Debug)]
pub enum FlowTypeCreationError {
    RegistrationError(FlowTypeRegistrationError),
}

#[derive(Debug)]
pub enum FlowTypeImportError {
    Io(std::io::Error),
    Deserialization(serde_json::Error),
    RegistrationError(FlowTypeRegistrationError),
}

impl From<std::io::Error> for FlowTypeImportError {
    fn from(e: std::io::Error) -> Self {
        FlowTypeImportError::Io(e)
    }
}

impl From<serde_json::Error> for FlowTypeImportError {
    fn from(e: serde_json::Error) -> Self {
        FlowTypeImportError::Deserialization(e)
    }
}

#[derive(Debug)]
pub enum FlowTypeExportError {
    FlowTypeNotFound(FlowTypeId),
    Io(std::io::Error),
    Serialization(serde_json::Error),
}
