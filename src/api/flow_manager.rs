use crate::api::entity_instance_manager::EntityInstanceCreationError;
use crate::api::RelationInstanceCreationError;
use async_trait::async_trait;

use crate::model::Flow;

#[derive(Debug)]
pub enum FlowCreationError {
    EntityInstanceCreationError(EntityInstanceCreationError),
    RelationInstanceCreationError(RelationInstanceCreationError),
}

impl From<EntityInstanceCreationError> for FlowCreationError {
    fn from(e: EntityInstanceCreationError) -> Self {
        FlowCreationError::EntityInstanceCreationError(e)
    }
}

impl From<RelationInstanceCreationError> for FlowCreationError {
    fn from(e: RelationInstanceCreationError) -> Self {
        FlowCreationError::RelationInstanceCreationError(e)
    }
}

#[derive(Debug)]
pub enum FlowImportError {
    Io(std::io::Error),
    Deserialization(serde_json::Error),
    FlowCreation(FlowCreationError),
}

impl From<std::io::Error> for FlowImportError {
    fn from(e: std::io::Error) -> Self {
        FlowImportError::Io(e)
    }
}

impl From<serde_json::Error> for FlowImportError {
    fn from(e: serde_json::Error) -> Self {
        FlowImportError::Deserialization(e)
    }
}

impl From<FlowCreationError> for FlowImportError {
    fn from(e: FlowCreationError) -> Self {
        FlowImportError::FlowCreation(e)
    }
}

#[async_trait]
pub trait FlowManager: Send + Sync {
    fn create(&self, flow: Flow) -> Result<Flow, FlowCreationError>;

    fn commit(&self, flow: Flow);

    fn delete(&self, flow: Flow);

    fn import(&self, path: String) -> Result<Flow, FlowImportError>;

    fn export(&self, flow: Flow, path: String);
}
