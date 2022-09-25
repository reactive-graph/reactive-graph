use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::{EntityType, Extension, PropertyType};
use crate::plugins::EntityTypeProvider;

#[derive(Debug)]
pub enum EntityTypeImportError {
    Io(std::io::Error),
    Deserialization(serde_json::Error),
}

impl From<std::io::Error> for EntityTypeImportError {
    fn from(e: std::io::Error) -> Self {
        EntityTypeImportError::Io(e)
    }
}

impl From<serde_json::Error> for EntityTypeImportError {
    fn from(e: serde_json::Error) -> Self {
        EntityTypeImportError::Deserialization(e)
    }
}

#[async_trait]
pub trait EntityTypeManager: Send + Sync + Lifecycle {
    fn register(&self, entity_type: EntityType) -> EntityType;

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
    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError>;

    /// Exports the entity type with the given name to a JSON file located at the given path.
    fn export(&self, name: &str, path: &str);

    /// Registers an entity type provider.
    fn add_provider(&self, entity_type_provider: Arc<dyn EntityTypeProvider>);

    /// Returns the entity type categories.
    fn get_entity_type_categories(&self) -> Vec<String>;
}
