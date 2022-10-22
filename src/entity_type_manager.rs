use crate::model::EntityType;
use crate::model::EntityTypeType;
use crate::model::Extension;
use crate::model::PropertyType;
use inexor_rgf_core_model::ComponentType;

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
    fn has(&self, ty: &EntityTypeType) -> bool;

    /// Returns true, if a entity type with the given fully qualified name exists.
    fn has_by_type(&self, namespace: &str, name: &str) -> bool;

    /// Returns the entity type with the given name or empty.
    fn get(&self, ty: &EntityTypeType) -> Option<EntityType>;

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
        ty: &EntityTypeType,
        description: &str,
        components: Vec<ComponentType>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<EntityType, EntityTypeCreationError>;

    /// Adds the component with the given component_name to the entity type with the given name.
    fn add_component(&self, ty: &EntityTypeType, component: &ComponentType);

    /// Remove the component with the given component_name from the entity type with the given name.
    fn remove_component(&self, ty: &EntityTypeType, component: &ComponentType);

    /// Adds a property to the entity type with the given name.
    fn add_property(&self, ty: &EntityTypeType, property: PropertyType);

    /// Removes the property with the given property_name from the entity type with the given name.
    fn remove_property(&self, ty: &EntityTypeType, property_name: &str);

    /// Adds an extension to the entity type with the given name.
    fn add_extension(&self, ty: &EntityTypeType, extension: Extension);

    /// Removes the extension with the given extension_name from the entity type with the given name.
    fn remove_extension(&self, ty: &EntityTypeType, extension_name: &str);

    /// Deletes the entity type with the given name.
    fn delete(&self, ty: &EntityTypeType);

    /// Validates the entity type with the given name.
    /// Tests that all components exists.
    fn validate(&self, ty: &EntityTypeType) -> bool;

    /// Imports an entity type from a JSON file file located at the given path.
    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError>;

    /// Exports the entity type with the given name to a JSON file located at the given path.
    fn export(&self, ty: &EntityTypeType, path: &str);
}
