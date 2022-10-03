use crate::model::Extension;
use crate::model::PropertyType;
use crate::model::RelationType;

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
    fn get_relation_types(&self) -> Vec<RelationType>;

    /// Returns all relation types of the given namespace.
    fn get_relation_types_by_namespace(&self, namespace: &str) -> Vec<RelationType>;

    /// Returns true, if a relation type with the given name exists.
    fn has(&self, type_name: &str) -> bool;

    /// Returns true, if a relation type exists whose name starts with the given name.
    fn has_starts_with(&self, type_name: &str) -> bool;

    /// Returns the relation type with the given name.
    fn get(&self, type_name: &str) -> Option<RelationType>;

    /// Returns the relation type whose name starts with the given name.
    fn get_starts_with(&self, type_name_starts_with: &str) -> Option<RelationType>;

    /// Returns all relation types whose names matches the given search string.
    fn find(&self, search: &str) -> Vec<RelationType>;

    /// Returns the count of relation types.
    fn count(&self) -> usize;

    /// Creates a new relation type.
    #[allow(clippy::too_many_arguments)]
    fn create(
        &self,
        namespace: &str,
        outbound_type: &str,
        type_name: &str,
        inbound_type: &str,
        description: &str,
        components: Vec<String>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<RelationType, RelationTypeCreationError>;

    /// Adds the component with the given component_name to the relation type with the given name.
    fn add_component(&self, name: &str, component_name: &str);

    /// Remove the component with the given component_name from the relation type with the given name.
    fn remove_component(&self, name: &str, component_name: &str);

    /// Adds a property to the relation type with the given name.
    fn add_property(&self, name: &str, property: PropertyType);

    /// Removes the property with the given property_name from the relation type with the given name.
    fn remove_property(&self, name: &str, property_name: &str);

    /// Adds an extension to the relation type with the given name.
    fn add_extension(&self, name: &str, extension: Extension);

    /// Removes the extension with the given extension_name from the relation type with the given name.
    fn remove_extension(&self, name: &str, extension_name: &str);

    /// Deletes the relation type with the given name.
    fn delete(&self, type_name: &str);

    /// Imports a relation type from a JSON file located at the given path.
    fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError>;

    /// Exports the relation type with the given name to a JSON file located at the given path.
    fn export(&self, type_name: &str, path: &str);
}
