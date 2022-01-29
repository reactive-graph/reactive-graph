use std::collections::HashMap;
use std::fmt;

use async_trait::async_trait;
use indradb::EdgeKey;
use serde_json::Value;
use uuid::Uuid;

use crate::api::RelationEdgeCreationError;
use crate::model::RelationInstance;

#[derive(Debug)]
pub enum RelationInstanceCreationError {
    InvalidEdgeKey,
    EdgeAlreadyExists(EdgeKey),
    MissingOutboundEntityInstance(Uuid),
    MissingInboundEntityInstance(Uuid),
    RelationEdgeCreationError(RelationEdgeCreationError),
}

impl fmt::Display for RelationInstanceCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            RelationInstanceCreationError::InvalidEdgeKey => write!(f, "The edge key is invalid"),
            RelationInstanceCreationError::EdgeAlreadyExists(edge_key) => {
                write!(f, "The edge already exists: {:?}", edge_key)
            }
            RelationInstanceCreationError::MissingOutboundEntityInstance(id) => {
                write!(f, "The outbound entity instance {} cannot be found", id)
            }
            RelationInstanceCreationError::MissingInboundEntityInstance(id) => {
                write!(f, "The inbound entity instance {} cannot be found", id)
            }
            RelationInstanceCreationError::RelationEdgeCreationError(error) => write!(f, "Failed to create relation instance: {}", error),
        }
    }
}

#[derive(Debug)]
pub enum RelationInstanceImportError {
    Io(std::io::Error),
    Deserialize(serde_json::Error),
    InvalidEdgeKey,
    RelationAlreadyExists(EdgeKey),
    RelationEdgeCreation(RelationEdgeCreationError),
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

#[async_trait]
pub trait RelationInstanceManager: Send + Sync {
    /// Returns true, if an relation instance exists with the given key.
    fn has(&self, edge_key: EdgeKey) -> bool;

    /// Returns the relation instance with the given key or None.
    fn get(&self, edge_key: EdgeKey) -> Option<RelationInstance>;

    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<RelationInstance>;

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<RelationInstance>;

    fn create(&self, edge_key: EdgeKey, properties: HashMap<String, Value>) -> Result<EdgeKey, RelationInstanceCreationError>;

    fn create_from_instance(&self, relation_instance: RelationInstance) -> Result<EdgeKey, RelationInstanceCreationError>;

    // TODO: return result
    fn commit(&self, relation_instance: RelationInstance);

    fn delete(&self, edge_key: EdgeKey) -> bool;

    fn import(&self, path: String) -> Result<RelationInstance, RelationInstanceImportError>;

    // TODO: return result
    // TODO: egde_key ?
    fn export(&self, edge_key: EdgeKey, path: String);
}
