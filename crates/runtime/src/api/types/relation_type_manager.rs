use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::ComponentTypeIds;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::Extensions;
use crate::model::Namespaces;
use crate::model::PropertyType;
use crate::model::PropertyTypes;
use crate::model::RelationType;
use crate::model::RelationTypeAddComponentError;
use crate::model::RelationTypeAddExtensionError;
use crate::model::RelationTypeAddPropertyError;
use crate::model::RelationTypeId;
use crate::model::RelationTypeIds;
use crate::model::RelationTypeMergeError;
use crate::model::RelationTypeRemoveComponentError;
use crate::model::RelationTypeRemoveExtensionError;
use crate::model::RelationTypeRemovePropertyError;
use crate::model::RelationTypeUpdateExtensionError;
use crate::model::RelationTypeUpdatePropertyError;
use crate::model::RelationTypes;
use crate::rt_api::RelationTypeCreationError;
use crate::rt_api::RelationTypeRegistrationError;

#[async_trait]
pub trait RelationTypeManager: Send + Sync + Lifecycle {
    fn register(&self, relation_type: RelationType) -> Result<RelationType, RelationTypeRegistrationError>;

    /// Returns all relation types.
    fn get_all(&self) -> RelationTypes;

    /// Returns the ids of all relation types.
    fn get_type_ids(&self) -> RelationTypeIds;

    /// Returns all defined namespaces.
    fn get_namespaces(&self) -> Namespaces;

    /// Returns all relation types of the given namespace
    fn get_by_namespace(&self, namespace: &str) -> RelationTypes;

    /// Returns all relation types of the given namespace
    fn get_types_by_namespace(&self, namespace: &str) -> RelationTypeIds;

    /// Returns all relation types of the given namespace
    fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> RelationTypes;

    /// Returns outbound relation types for the given entity type.
    fn get_outbound_relation_types(&self, outbound_ty: &ComponentOrEntityTypeId, wildcard: bool) -> RelationTypes;

    /// Returns inbound relation types for the given entity type.
    fn get_inbound_relation_types(&self, inbound_ty: &ComponentOrEntityTypeId, wildcard: bool) -> RelationTypes;

    /// Returns true, if a relation type with the given name exists.
    fn has(&self, ty: &RelationTypeId) -> bool;

    /// Returns true, if a relation type with the given fully qualified name exists.
    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool;

    /// Returns the relation type with the given name.
    fn get(&self, ty: &RelationTypeId) -> Option<RelationType>;

    /// Returns the relation type with the given fully qualified name.
    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<RelationType>;

    /// Returns all relation types whose names matches the given search string.
    fn find_by_type_name(&self, search: &str) -> RelationTypes;

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
        components: ComponentTypeIds,
        properties: PropertyTypes,
        extensions: Extensions,
    ) -> Result<RelationType, RelationTypeCreationError>;

    /// Merges the given relation_type_to_merge into an existing relation type with the same relation type id.
    fn merge(&self, relation_type_to_merge: RelationType) -> Result<RelationType, RelationTypeMergeError>;

    /// Adds the component with the given component_name to the relation type with the given name.
    fn add_component(&self, ty: &RelationTypeId, component_ty: &ComponentTypeId) -> Result<(), RelationTypeAddComponentError>;

    /// Remove the component with the given component_name from the relation type with the given name.
    fn remove_component(&self, ty: &RelationTypeId, component_ty: &ComponentTypeId) -> Result<ComponentTypeId, RelationTypeRemoveComponentError>;

    /// Adds a property to the relation type with the given name.
    fn add_property(&self, ty: &RelationTypeId, property: PropertyType) -> Result<PropertyType, RelationTypeAddPropertyError>;

    /// Updates the property with the given property_name.
    /// It's possible to rename the property by using another name in the new property than the provided property_name.
    fn update_property(
        &self,
        relation_ty: &RelationTypeId,
        property_name: &str,
        property_type: PropertyType,
    ) -> Result<PropertyType, RelationTypeUpdatePropertyError>;

    /// Removes the property with the given property_name from the relation type with the given name.
    fn remove_property(&self, ty: &RelationTypeId, property_name: &str) -> Result<PropertyType, RelationTypeRemovePropertyError>;

    /// Adds an extension to the relation type with the given name.
    fn add_extension(&self, ty: &RelationTypeId, extension: Extension) -> Result<ExtensionTypeId, RelationTypeAddExtensionError>;

    /// Updates the extension with the given extension type.
    /// It's possible to rename the extension by using another extension type in the new extension than the provided extension type.
    fn update_extension(
        &self,
        relation_ty: &RelationTypeId,
        extension_ty: &ExtensionTypeId,
        extension: Extension,
    ) -> Result<Extension, RelationTypeUpdateExtensionError>;

    /// Removes the extension with the given extension_name from the relation type with the given name.
    fn remove_extension(&self, relation_ty: &RelationTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, RelationTypeRemoveExtensionError>;

    /// Deletes the relation type with the given name.
    fn delete(&self, ty: &RelationTypeId) -> Option<RelationType>;

    /// Validates the relation type with the given name.
    /// Tests that all components, the outbound and inbound entity type exists.
    fn validate(&self, ty: &RelationTypeId) -> bool;
}
