use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::EntityType;
use crate::model::Extension;
use crate::model::PropertyType;
use crate::plugins::EntityTypeProvider;

#[derive(Debug)]
pub enum EntityTypeImportError {
    Io(std::io::Error),
    Deserialization(serde_json::Error),
}

#[derive(Debug)]
pub enum EntityTypeComponentError {
    ComponentAlreadyAssigned,
    ComponentDoesNotExist,
}

#[derive(Debug)]
pub enum EntityTypePropertyError {
    PropertyAlreadyExists,
}

#[derive(Debug)]
pub enum EntityTypeExtensionError {
    ExtensionAlreadyExists,
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

    /// Returns all entity types of the given namespace
    fn get_entity_types_by_namespace(&self, namespace: &str) -> Vec<EntityType>;

    /// Returns true, if a entity type with the given name exists.
    fn has(&self, name: &str) -> bool;

    /// Returns the entity type with the given name or empty.
    fn get(&self, name: &str) -> Option<EntityType>;

    /// Returns all entity types whose names matches the given search string.
    fn find(&self, search: &str) -> Vec<EntityType>;

    /// Returns the count of entity types.
    fn count(&self) -> usize;

    /// Creates a new entity type.
    fn create(&self, namespace: &str, name: &str, description: &str, components: Vec<String>, properties: Vec<PropertyType>, extensions: Vec<Extension>);

    /// Adds the component with the given component_name to the entity type with the given name.
    fn add_component(&self, name: &str, component_name: &str) -> Result<(), EntityTypeComponentError>;

    /// Remove the component with the given component_name from the entity type with the given name.
    fn remove_component(&self, name: &str, component_name: &str);

    /// Adds a property to the entity type with the given name.
    fn add_property(&self, name: &str, property: PropertyType) -> Result<(), EntityTypePropertyError>;

    /// Removes the property with the given property_name from the entity type with the given name.
    fn remove_property(&self, name: &str, property_name: &str);

    /// Adds an extension to the entity type with the given name.
    fn add_extension(&self, name: &str, extension: Extension) -> Result<(), EntityTypeExtensionError>;

    /// Removes the extension with the given extension_name from the entity type with the given name.
    fn remove_extension(&self, name: &str, extension_name: &str);

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
