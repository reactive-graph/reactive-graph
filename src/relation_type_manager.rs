use crate::model::ComponentTypeId;
use crate::model::EntityTypeId;
use crate::model::Extension;
use crate::model::PropertyType;
use crate::model::RelationType;
use crate::model::RelationTypeId;

#[derive(Debug)]
pub enum RelationTypeManagerError {
    InitializationError,
}

#[derive(Debug)]
pub enum RelationTypeCreationError {
    Failed,
}

#[derive(Debug)]
pub enum RelationTypeImportError {
    Failed,
}

pub trait RelationTypeManager: Send + Sync {
    /// Returns all relation types.
    fn get_all(&self) -> Vec<RelationType>;

    /// Returns all relation types of the given namespace.
    fn get_by_namespace(&self, namespace: &str) -> Vec<RelationType>;

    /// Returns true, if a relation type with the given name exists.
    fn has(&self, ty: &RelationTypeId) -> bool;

    /// Returns true, if a relation type with the given fully qualified name exists.
    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool;

    /// Returns true, if a relation type exists whose name starts with the given name.
    fn has_starts_with(&self, ty_starts_with: &RelationTypeId) -> bool;

    /// Returns true, if a relation type exists whose name starts with the given fully qualified name.
    fn has_by_type_starts_with(&self, namespace: &str, type_name_starts_with: &str) -> bool;

    /// Returns the relation type with the given name.
    fn get(&self, ty: &RelationTypeId) -> Option<RelationType>;

    /// Returns the relation type with the given fully qualified name.
    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<RelationType>;

    /// Returns the relation type whose name starts with the given name.
    fn get_starts_with(&self, ty_starts_with: &RelationTypeId) -> Option<RelationType>;

    /// Returns the relation type whose name starts with the given fully qualified name.
    fn get_by_type_starts_with(&self, namespace: &str, type_name_starts_with: &str) -> Option<RelationType>;

    /// Returns all relation types whose names matches the given search string.
    fn find(&self, search: &str) -> Vec<RelationType>;

    /// Returns the count of relation types.
    fn count(&self) -> usize;

    /// Returns the count of relation types of the given namespace.
    fn count_by_namespace(&self, namespace: &str) -> usize;

    /// Creates a new relation type.
    #[allow(clippy::too_many_arguments)]
    fn create(
        &self,
        outbound_type: &EntityTypeId,
        ty: &RelationTypeId,
        inbound_type: &EntityTypeId,
        description: &str,
        components: Vec<ComponentTypeId>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<RelationType, RelationTypeCreationError>;

    /// Adds the component with the given component_name to the relation type with the given name.
    fn add_component(&self, ty: &RelationTypeId, component: &ComponentTypeId);

    /// Remove the component with the given component_name from the relation type with the given name.
    fn remove_component(&self, ty: &RelationTypeId, component: &ComponentTypeId);

    /// Adds a property to the relation type with the given name.
    fn add_property(&self, ty: &RelationTypeId, property: PropertyType);

    /// Removes the property with the given property_name from the relation type with the given name.
    fn remove_property(&self, ty: &RelationTypeId, property_name: &str);

    /// Adds an extension to the relation type with the given name.
    fn add_extension(&self, ty: &RelationTypeId, extension: Extension);

    /// Removes the extension with the given extension_name from the relation type with the given name.
    fn remove_extension(&self, ty: &RelationTypeId, extension_name: &str);

    /// Deletes the relation type with the given name.
    fn delete(&self, ty: &RelationTypeId);

    /// Validates the relation type with the given name.
    /// Tests that all components, the outbound and inbound entity type exists.
    fn validate(&self, ty: &RelationTypeId) -> bool;

    /// Imports a relation type from a JSON file located at the given path.
    fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError>;

    /// Exports the relation type with the given name to a JSON file located at the given path.
    fn export(&self, ty: &RelationTypeId, path: &str);
}
