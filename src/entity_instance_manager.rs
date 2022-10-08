use std::collections::HashMap;
use std::sync::Arc;

use uuid::Uuid;

use crate::model::EntityInstance;
use crate::model::ReactiveEntityInstance;

#[derive(Debug)]
pub enum EntityInstanceManagerError {
    InitializationError,
}

#[derive(Debug)]
pub enum EntityInstanceCreationError {
    Failed,
}

#[derive(Debug)]
pub enum EntityInstanceComponentAddError {
    MissingComponent(String),
    MissingInstance(Uuid),
}

pub trait EntityInstanceManager: Send + Sync {
    /// Returns true, if an entity instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the reactive entity instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<Arc<ReactiveEntityInstance>>;

    /// Returns the reactive entity instance with the given label or None.
    fn get_by_label(&self, label: &str) -> Option<Arc<ReactiveEntityInstance>>;

    /// Returns the reactive entity instance and the matched path parameters that matches the given label or None.
    /// /org/inexor/local/users/:user_id
    /// /org/inexor/local/users/PeterPenacka returns: (instance, {"user_id": "PeterPenacka"})
    fn get_by_label_with_params(&self, label: &str) -> Option<(Arc<ReactiveEntityInstance>, HashMap<String, String>)>;

    /// Returns all reactive entity instances.
    fn get_all(&self) -> Vec<Arc<ReactiveEntityInstance>>;

    /// Returns all ids.
    fn get_ids(&self) -> Vec<Uuid>;

    /// Creates a new reactive entity instance.
    fn create(&self, entity_instance: EntityInstance) -> Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>;

    /// Adds the component with the given name to the entity instance with the given id.
    fn add_component(&self, id: Uuid, component: &str) -> Result<(), EntityInstanceComponentAddError>;

    /// Removes the component with the given name from the entity instance with the given id.
    fn remove_component(&self, id: Uuid, component: &str);

    /// Deletes the reactive entity instance with the given id.
    fn delete(&self, id: Uuid);
}
