use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::EntityTypeId;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::PropertyType;
use crate::model::RelationType;
use crate::model::RelationTypeId;
use crate::plugins::RelationTypeProvider;

#[derive(Debug)]
pub enum RelationTypeRegistrationError {
    RelationTypeAlreadyExists(RelationTypeId),
    OutboundComponentDoesNotExist(RelationTypeId, ComponentTypeId),
    OutboundEntityTypeDoesNotExist(RelationTypeId, EntityTypeId),
    InboundComponentDoesNotExist(RelationTypeId, ComponentTypeId),
    InboundEntityTypeDoesNotExist(RelationTypeId, EntityTypeId),
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
    ExtensionAlreadyExists(ExtensionTypeId),
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
    fn get_all(&self) -> Vec<RelationType>;

    /// Returns all relation types of the given namespace
    fn get_by_namespace(&self, namespace: &str) -> Vec<RelationType>;

    /// Returns all entity types of the given namespace
    fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> Vec<RelationType>;

    /// Returns outbound relation types for the given entity type.
    fn get_outbound_relation_types(&self, outbound_ty: &ComponentOrEntityTypeId, wildcard: bool) -> Vec<RelationType>;

    /// Returns inbound relation types for the given entity type.
    fn get_inbound_relation_types(&self, inbound_ty: &ComponentOrEntityTypeId, wildcard: bool) -> Vec<RelationType>;

    /// Returns true, if a relation type with the given name exists.
    fn has(&self, ty: &RelationTypeId) -> bool;

    /// Returns true, if a relation type with the given fully qualified name exists.
    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool;

    /// Returns the relation type with the given name.
    fn get(&self, ty: &RelationTypeId) -> Option<RelationType>;

    /// Returns the relation type with the given fully qualified name.
    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<RelationType>;

    /// Returns all relation types whose names matches the given search string.
    fn find(&self, search: &str) -> Vec<RelationType>;

    /// Returns the count of relation types.
    fn count(&self) -> usize;

    /// Returns the count of relation types of the given namespace.
    fn count_by_namespace(&self, namespace: &str) -> usize;

    /// Creates a new relation type.
    fn create(
        &self,
        outbound_type: &ComponentOrEntityTypeId,
        ty: &RelationTypeId,
        inbound_type: &ComponentOrEntityTypeId,
        description: &str,
        components: Vec<ComponentTypeId>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<RelationType, RelationTypeCreationError>;

    /// Adds the component with the given component_name to the relation type with the given name.
    fn add_component(&self, ty: &RelationTypeId, component_ty: &ComponentTypeId) -> Result<(), RelationTypeComponentError>;

    /// Remove the component with the given component_name from the relation type with the given name.
    fn remove_component(&self, ty: &RelationTypeId, component_ty: &ComponentTypeId);

    /// Adds a property to the relation type with the given name.
    fn add_property(&self, ty: &RelationTypeId, property: PropertyType) -> Result<(), RelationTypePropertyError>;

    /// Removes the property with the given property_name from the relation type with the given name.
    fn remove_property(&self, ty: &RelationTypeId, property_name: &str);

    /// Adds an extension to the relation type with the given name.
    fn add_extension(&self, ty: &RelationTypeId, extension: Extension) -> Result<(), RelationTypeExtensionError>;

    /// Removes the extension with the given extension_name from the relation type with the given name.
    fn remove_extension(&self, relation_ty: &RelationTypeId, extension_ty: &ExtensionTypeId);

    /// Deletes the relation type with the given name.
    fn delete(&self, ty: &RelationTypeId);

    /// Validates the relation type with the given name.
    /// Tests that all components, the outbound and inbound entity type exists.
    fn validate(&self, ty: &RelationTypeId) -> bool;

    /// Imports a relation type from a JSON file located at the given path.
    fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError>;

    /// Exports the relation type with the given name to a JSON file located at the given path.
    fn export(&self, ty: &RelationTypeId, path: &str);

    /// Registers a relation type provider.
    fn add_provider(&self, relation_type_provider: Arc<dyn RelationTypeProvider>);
}
