use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;

use crate::api::{EntityInstanceCreationError, EntityInstanceImportError, Lifecycle};
use crate::model::{EntityInstance, ReactiveEntityInstance};

#[derive(Debug)]
pub enum ReactiveEntityInstanceCreationError {
    UuidTaken(Uuid),
    MissingInstance,
    EntityInstanceCreationError(EntityInstanceCreationError),
}

impl fmt::Display for ReactiveEntityInstanceCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ReactiveEntityInstanceCreationError::UuidTaken(id) => {
                write!(f, "The UUID {} has been already taken!", id)
            }
            ReactiveEntityInstanceCreationError::MissingInstance => {
                write!(f, "The created instance cannot be found")
            }
            ReactiveEntityInstanceCreationError::EntityInstanceCreationError(error) => {
                write!(f, "Failed to create reactive entity instance: {}", error)
            }
        }
    }
}

#[derive(Debug)]
pub enum ReactiveEntityInstanceImportError {
    EntityInstanceImport(EntityInstanceImportError),
    MissingEntityInstance(Uuid),
    ReactiveEntityInstanceCreation(ReactiveEntityInstanceCreationError),
}

#[async_trait]
pub trait ReactiveEntityInstanceManager: Send + Sync + Lifecycle {
    /// Returns true, if an entity instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the reactive entity instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<Arc<ReactiveEntityInstance>>;

    /// Returns the reactive entity instance that matches the given label or None.
    fn get_by_label(&self, label: String) -> Option<Arc<ReactiveEntityInstance>>;

    /// Returns the reactive entity instance and the matched path parameters that matches the given label or None.
    /// /org/inexor/local/users/:user_id
    /// /org/inexor/local/users/PeterPenacka returns: (instance, {"user_id": "PeterPenacka"})
    fn get_by_label_with_params(&self, label: String) -> Option<(Arc<ReactiveEntityInstance>, HashMap<String, String>)>;

    /// Returns all registered reactive entity instances.
    fn get_entity_instances(&self) -> Vec<Arc<ReactiveEntityInstance>>;

    /// Returns the ids of all registered reactive entity instances.
    fn get_ids(&self) -> Vec<Uuid>;

    // fn get_all(&self) -> Option<Arc<ReactiveEntityInstance>>;

    // fn get_by_type(&self, type_name: String) -> Option<Arc<ReactiveEntityInstance>>;

    // fn get_by_property(&self, property_name: String, value: Value) -> Option<Arc<ReactiveEntityInstance>>;

    /// Creates a new reactive entity instance of the given type. The reactive instance will be
    /// initialized with the given properties and values. A random id will be generated.
    fn create(&self, type_name: String, properties: HashMap<String, Value>) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError>;

    /// Creates a new reactive entity instance of the given type, with the given id and initialized
    /// with the given properties and values.
    fn create_with_id(
        &self,
        type_name: String,
        id: Uuid,
        properties: HashMap<String, Value>,
    ) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError>;

    /// Creates a reactive entity instance from the given non-reactive entity instance. The
    /// reactive entity instance will be registered.
    fn create_reactive_instance(&self, entity_instance: EntityInstance) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError>;

    /// Registers a reactive entity instance.
    fn register_reactive_instance(&self, reactive_entity_instance: Arc<ReactiveEntityInstance>);

    /// Registers a reactive entity instance if and only if the given instance doesn't exist.
    ///
    /// No properties are merged if the given entity instance already exists.
    fn register_or_merge_reactive_instance(&self, reactive_entity_instance: Arc<ReactiveEntityInstance>) -> Arc<ReactiveEntityInstance>;

    /// Adds the component with the given name to the entity instance with the given id.
    fn add_component(&self, id: Uuid, component: String);

    /// Removes the component with the given name from the entity instance with the given id.
    fn remove_component(&self, id: Uuid, component: String);

    // TODO: return result
    fn commit(&self, id: Uuid);

    fn delete(&self, id: Uuid);

    // TODO: fn delete_and_delete_relations(&self, id: Uuid);

    /// Unregisters the reactive entity instance. Also removes all behaviours. If there are any
    /// references to the reactive entity instance, their reactive streams still work but the
    /// applied behaviours are gone.
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
