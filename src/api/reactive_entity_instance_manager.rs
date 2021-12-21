use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;

use crate::api::EntityInstanceCreationError;
use crate::model::{EntityInstance, ReactiveEntityInstance};

#[derive(Debug)]
pub enum ReactiveEntityInstanceCreationError {
    UuidTaken(Uuid),
    MissingInstance,
    EntityInstanceCreationError(EntityInstanceCreationError),
}

impl fmt::Display for ReactiveEntityInstanceCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            ReactiveEntityInstanceCreationError::UuidTaken(id) => {
                write!(f, "The UUID {} has been already taken!", id)
            }
            ReactiveEntityInstanceCreationError::MissingInstance => {
                write!(f, "The created instance cannot be found")
            }
            ReactiveEntityInstanceCreationError::EntityInstanceCreationError(error) => {
                write!(f, "Failed to create reactive entity instance: {}", error.to_string())
            }
        }
    }
}

#[derive(Debug)]
pub struct ReactiveEntityInstanceImportError;

#[async_trait]
pub trait ReactiveEntityInstanceManager: Send + Sync {
    /// Returns true, if an entity instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the reactive entity instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<Arc<ReactiveEntityInstance>>;

    fn get_entity_instances(&self) -> Vec<Arc<ReactiveEntityInstance>>;
    // fn get_all(&self) -> Option<Arc<ReactiveEntityInstance>>;

    // fn get_by_type(&self, type_name: String) -> Option<Arc<ReactiveEntityInstance>>;

    // fn get_by_property(&self, property_name: String, value: Value) -> Option<Arc<ReactiveEntityInstance>>;

    fn create(&self, type_name: String, properties: HashMap<String, Value>) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError>;

    fn create_with_id(
        &self,
        type_name: String,
        id: Uuid,
        properties: HashMap<String, Value>,
    ) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError>;

    fn create_reactive_instance(&self, entity_instance: EntityInstance) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError>;

    fn register_reactive_instance(&self, reactive_entity_instance: Arc<ReactiveEntityInstance>);

    // TODO: return result
    fn commit(&self, id: Uuid);

    fn delete(&self, id: Uuid);

    // TODO: fn delete_and_delete_relations(&self, id: Uuid);

    fn unregister_reactive_instance(&self, id: Uuid);

    // TODO: rename import_from_file
    fn import(&self, path: String) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceImportError>;

    // TODO: import_from_json_string
    // Goal: web-ui: upload entity instance as json file

    // TODO: return result
    // TODO: rename export_as_file
    fn export(&self, id: Uuid, path: String);

    // TODO: implement export_as_json_string
    // Goal: web-ui: download entity instance as json file
    // fn export_as_json_string(&self, id: Uuid) -> String;
}
