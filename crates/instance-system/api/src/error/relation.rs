// use crate::ReactiveRelationCreationError;
// use reactive_graph_graph::RelationInstanceId;

use reactive_graph_graph::RelationInstanceId;
use reactive_graph_reactive_service_api::ReactiveRelationCreationError;

#[derive(Debug)]
pub enum RelationInstanceImportError {
    Io(std::io::Error),
    Deserialize(serde_json::Error),
    RelationAlreadyExists(RelationInstanceId),
    ReactiveRelationCreationError(ReactiveRelationCreationError),
}

impl From<std::io::Error> for RelationInstanceImportError {
    fn from(e: std::io::Error) -> Self {
        RelationInstanceImportError::Io(e)
    }
}

impl From<serde_json::Error> for RelationInstanceImportError {
    fn from(e: serde_json::Error) -> Self {
        RelationInstanceImportError::Deserialize(e)
    }
}

#[derive(Debug)]
pub enum RelationInstanceExportError {
    RelationNotFound(RelationInstanceId),
    Io(String),
    Serialization(serde_json::Error),
}
