use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use indradb::{EdgeKey, ValidationError};
use serde_json::Value;
use uuid::Uuid;

use crate::api::{RelationInstanceCreationError, RelationInstanceImportError};
use crate::model::{ReactiveRelationInstance, RelationInstance};

#[derive(Debug)]
pub enum ReactiveRelationInstanceCreationError {
    InvalidEdgeKey,
    MissingOutboundEntityInstance(Uuid),
    MissingInboundEntityInstance(Uuid),
    MissingInstance,
    RelationInstanceCreationError(RelationInstanceCreationError),
    ValidationError(ValidationError),
}

impl fmt::Display for ReactiveRelationInstanceCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            // ReactiveRelationInstanceCreationError::UuidTaken(id) => write!(f, "The UUID {} has been already taken!", id),
            ReactiveRelationInstanceCreationError::InvalidEdgeKey => {
                write!(f, "The edge key is invalid")
            }
            ReactiveRelationInstanceCreationError::MissingOutboundEntityInstance(id) => {
                write!(f, "The outbound entity instance {} cannot be found", id)
            }
            ReactiveRelationInstanceCreationError::MissingInboundEntityInstance(id) => {
                write!(f, "The inbound entity instance {} cannot be found", id)
            }
            ReactiveRelationInstanceCreationError::MissingInstance => {
                write!(f, "The created instance cannot be found")
            }
            ReactiveRelationInstanceCreationError::RelationInstanceCreationError(error) => {
                write!(f, "Failed to create reactive relation instance: {}", error)
            }
            ReactiveRelationInstanceCreationError::ValidationError(error) => {
                write!(f, "Validation Error: {}", error)
            }
        }
    }
}

#[derive(Debug)]
pub enum ReactiveRelationInstanceImportError {
    RelationInstanceImport(RelationInstanceImportError),
    ReactiveRelationInstanceCreation(ReactiveRelationInstanceCreationError),
}

#[async_trait]
pub trait ReactiveRelationInstanceManager: Send + Sync {
    /// Returns true, if an relation of the given type exists which starts at the given outbound entity and
    /// ends at the given inbound entity.
    fn has(&self, edge_key: EdgeKey) -> bool;

    /// Returns the ReactiveRelationInstance with the given type_name, which starts at the given
    /// outbound entity and ends at the given inbound entity.
    fn get(&self, edge_key: EdgeKey) -> Option<Arc<ReactiveRelationInstance>>;

    /// Returns all reactive relation instances of the given outbound entity instance.
    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>>;

    /// Returns all reactive relation instances of the given inbound entity instance.
    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>>;

    // TODO: Rename to: "get_all"
    fn get_relation_instances(&self) -> Vec<Arc<ReactiveRelationInstance>>;

    /// Returns all edge keys.
    fn get_keys(&self) -> Vec<EdgeKey>;

    /// Creates a new reactive relation instance.
    fn create(&self, edge_key: EdgeKey, properties: HashMap<String, Value>) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError>;

    fn create_reactive_instance(&self, relation_instance: RelationInstance) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError>;

    fn register_reactive_instance(&self, reactive_relation_instance: Arc<ReactiveRelationInstance>);

    fn register_or_merge_reactive_instance(&self, reactive_relation_instance: Arc<ReactiveRelationInstance>) -> Arc<ReactiveRelationInstance>;

    /// Adds the component with the given name to the relation instance with the given edge key.
    fn add_component(&self, edge_key: EdgeKey, component: String);

    /// Removes the component with the given name from the relation instance with the given edge key.
    fn remove_component(&self, edge_key: EdgeKey, component: String);

    // TODO: fn commit(&self, relation_instance: RelationInstance);
    // TODO: return result
    fn commit(&self, edge_key: EdgeKey);

    /// Deletes the reactive relation instance with the given key.
    fn delete(&self, edge_key: EdgeKey) -> bool;

    fn unregister_reactive_instance(&self, edge_key: EdgeKey);

    fn import(&self, path: String) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceImportError>;

    // TODO: return result
    fn export(&self, edge_key: EdgeKey, path: String);
}
