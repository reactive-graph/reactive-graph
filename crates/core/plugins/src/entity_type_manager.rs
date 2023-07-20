use crate::model::ComponentTypeId;
use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::PropertyType;

#[derive(Debug)]
pub enum EntityTypeManagerError {
    InitializationError,
}

#[derive(Debug)]
pub enum EntityTypeCreationError {
    Failed,
}

#[derive(Debug)]
pub enum EntityTypeImportError {
    Failed,
}

pub trait EntityTypeManager: Send + Sync {
    /// Returns all entity types.
    fn get_all(&self) -> Vec<EntityType>;

    /// Returns all entity types of the given namespace.
    fn get_by_namespace(&self, namespace: &str) -> Vec<EntityType>;

    /// Returns true, if a entity type with the given name exists.
    fn has(&self, ty: &EntityTypeId) -> bool;

    /// Returns true, if a entity type with the given fully qualified name exists.
    fn has_by_type(&self, namespace: &str, name: &str) -> bool;

    /// Returns the entity type with the given name or empty.
    fn get(&self, ty: &EntityTypeId) -> Option<EntityType>;

    /// Returns the entity type with the given fully qualified name or empty.
    fn get_by_type(&self, namespace: &str, name: &str) -> Option<EntityType>;

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

    /// Adds the component with the given component_name to the given entity type.
    fn add_component(&self, ty: &EntityTypeId, component: &ComponentTypeId);

    /// Remove the component with the given component_name from the given entity type.
    fn remove_component(&self, ty: &EntityTypeId, component: &ComponentTypeId);

    /// Adds a property to the given entity type.
    fn add_property(&self, ty: &EntityTypeId, property: PropertyType);

    /// Removes the property with the given property_name from the given entity type.
    fn remove_property(&self, ty: &EntityTypeId, property_name: &str);

    /// Adds an extension to the given entity type.
    fn add_extension(&self, ty: &EntityTypeId, extension: Extension);

    /// Removes the extension with the given type from the given entity type.
    fn remove_extension(&self, entity_ty: &EntityTypeId, extension_ty: &ExtensionTypeId);

    /// Deletes the entity type.
    fn delete(&self, ty: &EntityTypeId) -> bool;

    /// Validates the entity type with the given name.
    /// Tests that all components exists.
    fn validate(&self, ty: &EntityTypeId) -> bool;

    /// Imports an entity type from a JSON file file located at the given path.
    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError>;

    /// Exports the entity type with the given name to a JSON file located at the given path.
    fn export(&self, ty: &EntityTypeId, path: &str);
}
