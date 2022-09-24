use crate::model::EntityType;
use crate::model::Extension;
use crate::model::PropertyType;

#[derive(Debug)]
pub enum EntityTypeManagerError {
    InitializationError,
}

#[derive(Debug)]
pub enum EntityTypeCreationError {
    Failed,
}

pub trait EntityTypeManager: Send + Sync {
    /// Returns all entity types.
    fn get_entity_types(&self) -> Vec<EntityType>;

    /// Returns true, if a entity type with the given name exists.
    fn has(&self, name: &str) -> bool;

    /// Returns the entity type with the given name or empty.
    fn get(&self, name: &str) -> Option<EntityType>;

    /// Returns all entity types whose names matches the given search string.
    fn find(&self, search: &str) -> Vec<EntityType>;

    /// Returns the count of entity types.
    fn count(&self) -> usize;

    /// Creates a new entity type.
    fn create(&self, name: String, namespace: String, components: Vec<String>, properties: Vec<PropertyType>, extensions: Vec<Extension>);

    /// Deletes the entity type with the given name.
    fn delete(&self, name: &str);

    /// Imports an entity type from a JSON file file located at the given path.
    fn import(&self, path: &str);

    /// Exports the entity type with the given name to a JSON file located at the given path.
    fn export(&self, name: &str, path: &str);
}
