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

pub trait RelationTypeManager: Send + Sync {
    /// Returns all relation types.
    fn get_relation_types(&self) -> Vec<RelationType>;

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
    fn create(
        &self,
        outbound_type: String,
        type_name: String,
        inbound_type: String,
        components: Vec<String>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    );

    /// Deletes the relation type with the given name.
    fn delete(&self, type_name: &str);

    /// Imports a relation type from a JSON file located at the given path.
    fn import(&self, path: &str);

    /// Exports the relation type with the given name to a JSON file located at the given path.
    fn export(&self, type_name: &str, path: &str);
}
