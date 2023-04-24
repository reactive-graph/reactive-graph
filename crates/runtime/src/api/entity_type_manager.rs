use std::collections::HashSet;
use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::ComponentTypeId;
use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::PropertyType;
use crate::plugins::EntityTypeProvider;

#[derive(Debug)]
pub enum EntityTypeRegistrationError {
    EntityTypeAlreadyExists(EntityTypeId),
}

#[derive(Debug)]
pub enum EntityTypeMergeError {
    EntityTypeDoesNotExists(EntityTypeId),
}

#[derive(Debug)]
pub enum EntityTypeCreationError {
    RegistrationError(EntityTypeRegistrationError),
}

#[derive(Debug)]
pub enum EntityTypeImportError {
    Io(std::io::Error),
    Deserialization(serde_json::Error),
    RegistrationError(EntityTypeRegistrationError),
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
    ExtensionAlreadyExists(ExtensionTypeId),
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
    fn register(&self, entity_type: EntityType) -> Result<EntityType, EntityTypeRegistrationError>;

    /// Returns all entity types.
    fn get_all(&self) -> Vec<EntityType>;

    /// Returns all defined namespaces.
    fn get_namespaces(&self) -> HashSet<String>;

    /// Returns all entity types of the given namespace
    fn get_by_namespace(&self, namespace: &str) -> Vec<EntityType>;

    /// Returns all entity types of the given namespace
    fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> Vec<EntityType>;

    /// Returns true, if a entity type with the given name exists.
    fn has(&self, ty: &EntityTypeId) -> bool;

    /// Returns true, if a entity type with the given fully qualified name exists.
    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool;

    /// Returns the entity type with the given name or empty.
    fn get(&self, ty: &EntityTypeId) -> Option<EntityType>;

    /// Returns the entity type with the given fully qualified name or empty.
    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<EntityType>;

    /// Returns all entity types whose names matches the given search string.
    fn find(&self, search: &str) -> Vec<EntityType>;

    /// Returns the count of entity types.
    fn count(&self) -> usize;

    /// Returns the count of entity types of the given namespace.
    fn count_by_namespace(&self, namespace: &str) -> usize;

    /// Creates a new entity type.
    fn create(
        &self,
        ty: &EntityTypeId,
        description: &str,
        components: Vec<ComponentTypeId>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<EntityType, EntityTypeCreationError>;

    /// Merges the given entity_type_to_merge into an existing entity type with the same entity type id.
    fn merge(&self, entity_type_to_merge: EntityType) -> Result<EntityType, EntityTypeMergeError>;

    /// Adds the component with the given component_name to the entity type with the given name.
    fn add_component(&self, ty: &EntityTypeId, component: &ComponentTypeId) -> Result<(), EntityTypeComponentError>;

    /// Remove the component with the given component_name from the entity type with the given name.
    fn remove_component(&self, ty: &EntityTypeId, component: &ComponentTypeId);

    /// Adds a property to the entity type with the given name.
    fn add_property(&self, ty: &EntityTypeId, property: PropertyType) -> Result<(), EntityTypePropertyError>;

    /// Removes the property with the given property_name from the entity type with the given name.
    fn remove_property(&self, ty: &EntityTypeId, property_name: &str);

    /// Adds an extension to the entity type with the given name.
    fn add_extension(&self, ty: &EntityTypeId, extension: Extension) -> Result<(), EntityTypeExtensionError>;

    /// Removes the extension with the given extension_name from the entity type with the given name.
    fn remove_extension(&self, entity_ty: &EntityTypeId, extension_ty: &ExtensionTypeId);

    /// Deletes the entity type with the given name.
    fn delete(&self, ty: &EntityTypeId);

    /// Validates the entity type with the given name.
    /// Tests that all components exists.
    fn validate(&self, ty: &EntityTypeId) -> bool;

    /// Imports an entity type from a JSON file file located at the given path.
    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError>;

    /// Exports the entity type with the given name to a JSON file located at the given path.
    fn export(&self, ty: &EntityTypeId, path: &str);

    /// Registers an entity type provider.
    fn add_provider(&self, entity_type_provider: Arc<dyn EntityTypeProvider>);
}
