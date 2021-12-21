use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::{EntityType, Extension, PropertyType};
use crate::plugins::EntityTypeProvider;

#[derive(Debug)]
pub struct EntityTypeImportError;

#[async_trait]
pub trait EntityTypeManager: Send + Sync + Lifecycle {
    fn register(&self, entity_type: EntityType) -> EntityType;

    /// Returns all entity types.
    fn get_entity_types(&self) -> Vec<EntityType>;

    /// Returns true, if a entity type with the given name exists.
    fn has(&self, name: String) -> bool;

    /// Returns the entity type with the given name or empty.
    fn get(&self, name: String) -> Option<EntityType>;

    /// Creates a new entity type.
    fn create(&self, name: String, group: String, components: Vec<String>, behaviours: Vec<String>, properties: Vec<PropertyType>, extensions: Vec<Extension>);

    /// Deletes the entity type with the given name.
    fn delete(&self, name: String);

    fn import(&self, path: String) -> Result<EntityType, EntityTypeImportError>;
    fn export(&self, name: String, path: String);

    fn add_provider(&self, entity_type_provider: Arc<dyn EntityTypeProvider>);
}
