use std::collections::HashMap;
use std::sync::Arc;

use uuid::Uuid;

use crate::model::ComponentType;
use crate::model::EntityInstance;
use crate::model::EntityTypeType;
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

    /// Returns all reactive entity instances of the given type.
    fn get_by_type(&self, ty: &EntityTypeType) -> Vec<Arc<ReactiveEntityInstance>>;

    /// Returns all ids.
    fn get_ids(&self) -> Vec<Uuid>;

    /// Returns the count of registered reactive entity instances.
    fn count(&self) -> usize;

    /// Returns the count of registered reactive entity instances of the given type.
    fn count_by_type(&self, ty: &EntityTypeType) -> usize;

    /// Returns the count of registered reactive entity instances which are of the given component.
    fn count_by_component(&self, component: &ComponentType) -> usize;

    /// Returns the count of registered reactive entity instances which behaves as the given behaviour.
    fn count_by_behaviour(&self, behaviour: &str) -> usize;

    /// Creates a new reactive entity instance.
    fn create(&self, entity_instance: EntityInstance) -> Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>;

    /// Adds the component with the given name to the entity instance with the given id.
    fn add_component(&self, id: Uuid, component: &ComponentType) -> Result<(), EntityInstanceComponentAddError>;

    /// Removes the component with the given name from the entity instance with the given id.
    fn remove_component(&self, id: Uuid, component: &ComponentType);

    /// Deletes the reactive entity instance with the given id.
    fn delete(&self, id: Uuid);
}
