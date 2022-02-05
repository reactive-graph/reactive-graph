use std::sync::Arc;

use crate::model::{EntityInstance, ReactiveEntityInstance};
use uuid::Uuid;

#[derive(Debug)]
pub enum EntityInstanceCreationError {
    Failed,
}

pub trait EntityInstanceManager: Send + Sync {
    /// Returns true, if an entity instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the reactive entity instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<Arc<ReactiveEntityInstance>>;

    /// Returns all reactive entity instances.
    fn get_all(&self) -> Vec<Arc<ReactiveEntityInstance>>;

    /// Returns all ids.
    fn get_ids(&self) -> Vec<Uuid>;

    /// Creates a new reactive entity instance.
    fn create(&self, entity_instance: EntityInstance) -> Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>;

    /// Deletes the reactive entity instance with the given id.
    fn delete(&self, id: Uuid);
}
