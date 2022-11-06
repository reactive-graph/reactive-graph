use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::Extension;
use crate::model::PropertyType;
use crate::model::RelationType;
use crate::plugins::RelationTypeProvider;

#[derive(Debug)]
pub enum RelationTypeRegistrationError {
    RelationTypeAlreadyExists(String, String),
    OutboundEntityTypeDoesNotExist(String, String, String),
    InboundEntityTypeDoesNotExist(String, String, String),
}

#[derive(Debug)]
pub enum RelationTypeCreationError {
    RegistrationError(RelationTypeRegistrationError),
}

#[derive(Debug)]
pub enum RelationTypeImportError {
    Io(std::io::Error),
    Deserialize(serde_json::Error),
    RegistrationError(RelationTypeRegistrationError),
}

#[derive(Debug)]
pub enum RelationTypeComponentError {
    ComponentAlreadyAssigned,
    ComponentDoesNotExist,
}

#[derive(Debug)]
pub enum RelationTypePropertyError {
    PropertyAlreadyExists,
}

#[derive(Debug)]
pub enum RelationTypeExtensionError {
    ExtensionAlreadyExists,
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
    fn register(&self, relation_type: RelationType) -> Result<RelationType, RelationTypeRegistrationError>;

    /// Returns all relation types.
    fn get_relation_types(&self) -> Vec<RelationType>;

    /// Returns all relation types of the given namespace
    fn get_relation_types_by_namespace(&self, namespace: &str) -> Vec<RelationType>;

    /// Returns outbound relation types for the given entity type.
    fn get_outbound_relation_types(&self, entity_type_name: &str, wildcard: bool) -> Vec<RelationType>;

    /// Returns inbound relation types for the given entity type.
    fn get_inbound_relation_types(&self, entity_type_name: &str, wildcard: bool) -> Vec<RelationType>;

    /// Returns true, if a relation type with the given name exists.
    fn has(&self, type_name: &str) -> bool;

    /// Returns true, if a relation type with the given fully qualified name exists.
    fn has_fully_qualified(&self, namespace: &str, type_name: &str) -> bool;

    /// Returns true, if a relation type exists whose name starts with the given name.
    fn has_starts_with(&self, type_name: &str) -> bool;

    /// Returns the relation type with the given name.
    fn get(&self, type_name: &str) -> Option<RelationType>;

    /// Returns the relation type with the given fully qualified name.
    fn get_fully_qualified(&self, namespace: &str, type_name: &str) -> Option<RelationType>;

    /// Returns the relation type whose name starts with the given name.
    fn get_starts_with(&self, type_name_starts_with: &str) -> Option<RelationType>;

    /// Returns all relation types whose names matches the given search string.
    fn find(&self, search: &str) -> Vec<RelationType>;

    /// Returns the count of relation types.
    fn count(&self) -> usize;

    /// Creates a new relation type.
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
    fn add_component(&self, name: &str, component_name: &str) -> Result<(), RelationTypeComponentError>;

    /// Remove the component with the given component_name from the relation type with the given name.
    fn remove_component(&self, name: &str, component_name: &str);

    /// Adds a property to the relation type with the given name.
    fn add_property(&self, type_name: &str, property: PropertyType) -> Result<(), RelationTypePropertyError>;

    /// Removes the property with the given property_name from the relation type with the given name.
    fn remove_property(&self, type_name: &str, property_name: &str);

    /// Adds an extension to the relation type with the given name.
    fn add_extension(&self, type_name: &str, extension: Extension) -> Result<(), RelationTypeExtensionError>;

    /// Removes the extension with the given extension_name from the relation type with the given name.
    fn remove_extension(&self, type_name: &str, extension_name: &str);

    /// Deletes the relation type with the given name.
    fn delete(&self, type_name: &str);

    /// Imports a relation type from a JSON file located at the given path.
    fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError>;

    /// Exports the relation type with the given name to a JSON file located at the given path.
    fn export(&self, type_name: &str, path: &str);

    /// Registers a relation type provider.
    fn add_provider(&self, relation_type_provider: Arc<dyn RelationTypeProvider>);
}