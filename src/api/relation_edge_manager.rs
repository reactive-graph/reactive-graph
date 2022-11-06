use std::collections::HashMap;
use std::fmt;

use async_trait::async_trait;
use indradb::Edge;
use indradb::EdgeKey;
use indradb::EdgeProperties;
use serde_json::Value;
use uuid::Uuid;

use crate::model::RelationTypeId;

#[derive(Debug)]
pub enum RelationEdgeCreationError {
    InvalidEdgeKey(String),
    RelationTypeMissing(RelationTypeId),
    MissingRequiredProperty(String),
    GraphDatabaseError(indradb::Error),
}

impl fmt::Display for RelationEdgeCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            RelationEdgeCreationError::InvalidEdgeKey(edge_key) => {
                write!(f, "Invalid edge key {} does not exist!", edge_key)
            }
            RelationEdgeCreationError::RelationTypeMissing(ty) => {
                write!(f, "Relation type {} does not exist!", ty)
            }
            RelationEdgeCreationError::MissingRequiredProperty(property_name) => {
                write!(f, "Missing required property {}!", property_name)
            }
            RelationEdgeCreationError::GraphDatabaseError(error) => write!(f, "Failed to create graph database edge: {}", error),
        }
    }
}

#[async_trait]
pub trait RelationEdgeManager: Send + Sync {
    /// Returns true, if an relation instance edge exists with the given UUID.
    fn has(&self, edge_key: &EdgeKey) -> bool;

    /// Returns the edge by UUID.
    fn get(&self, edge_key: &EdgeKey) -> Option<Edge>;

    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<Edge>;

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<Edge>;

    /// Returns the edge properties by UUID. The result contains
    /// the edge and the type.
    fn get_properties(&self, edge_key: &EdgeKey) -> Option<EdgeProperties>;

    /// Creates a new edge with the given edge key and the given properties.
    fn create(&self, edge_key: &EdgeKey, properties: HashMap<String, Value>) -> Result<EdgeKey, RelationEdgeCreationError>;

    // TODO: return result RelationEdgeUpdateError
    // TODO: rename commit -> "update" or "save"
    fn commit(&self, edge_key: &EdgeKey, properties: HashMap<String, Value>);

    /// Deletes the edge with the given edge key.
    // TODO: return result RelationEdgeDeletionError
    fn delete(&self, edge_key: &EdgeKey) -> bool;
}
