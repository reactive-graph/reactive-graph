use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::{Extension, PropertyType, RelationType};
use crate::plugins::RelationTypeProvider;

#[derive(Debug)]
pub enum RelationTypeImportError {
    Io(std::io::Error),
    Deserialize(serde_json::Error),
}

impl From<std::io::Error> for RelationTypeImportError {
    fn from(e: std::io::Error) -> Self {
        RelationTypeImportError::Io(e)
    }
}

impl From<serde_json::Error> for RelationTypeImportError {
    fn from(e: serde_json::Error) -> Self {
        RelationTypeImportError::Deserialize(e)
    }
}

#[async_trait]
pub trait RelationTypeManager: Send + Sync + Lifecycle {
    // TODO: Result
    fn register(&self, relation_type: RelationType);

    /// Returns all relation types.
    fn get_relation_types(&self) -> Vec<RelationType>;

    /// Returns true, if a relation type with the given name exists.
    fn has(&self, type_name: String) -> bool;

    /// Returns true, if a relation type exists whose name starts with the given name.
    fn has_starts_with(&self, type_name: String) -> bool;

    /// Returns the relation type with the given name.
    fn get(&self, type_name: String) -> Option<RelationType>;

    /// Returns the relation type whose name starts with the given name.
    fn get_starts_with(&self, type_name_starts_with: String) -> Option<RelationType>;

    /// Returns all relation types whose names matches the given search string.
    fn find(&self, search: String) -> Vec<RelationType>;

    /// Creates a new relation type.
    // TODO: Result
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
    fn delete(&self, type_name: String);

    /// Imports a relation type from a JSON file located at the given path.
    fn import(&self, path: String) -> Result<RelationType, RelationTypeImportError>;

    /// Exports the relation type with the given name to a JSON file located at the given path.
    fn export(&self, type_name: String, path: String);

    /// Registers a relation type provider.
    fn add_provider(&self, relation_type_provider: Arc<dyn RelationTypeProvider>);
}
