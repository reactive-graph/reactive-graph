use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::ComponentTypeId;
use crate::model::ComponentTypeIds;
use crate::model::EntityType;
use crate::model::EntityTypeAddComponentError;
use crate::model::EntityTypeAddExtensionError;
use crate::model::EntityTypeAddPropertyError;
use crate::model::EntityTypeId;
use crate::model::EntityTypeIds;
use crate::model::EntityTypeMergeError;
use crate::model::EntityTypeRemoveComponentError;
use crate::model::EntityTypeRemoveExtensionError;
use crate::model::EntityTypeRemovePropertyError;
use crate::model::EntityTypeUpdateExtensionError;
use crate::model::EntityTypeUpdatePropertyError;
use crate::model::EntityTypes;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::Extensions;
use crate::model::Namespaces;
use crate::model::PropertyType;
use crate::model::PropertyTypes;
use crate::rt_api::EntityTypeCreationError;
use crate::rt_api::EntityTypeRegistrationError;

#[async_trait]
pub trait EntityTypeManager: Send + Sync + Lifecycle {
    fn register(&self, entity_type: EntityType) -> Result<EntityType, EntityTypeRegistrationError>;

    /// Returns all entity types.
    fn get_all(&self) -> EntityTypes;

    /// Returns the ids of all entity types.
    fn get_type_ids(&self) -> EntityTypeIds;

    /// Returns all defined namespaces.
    fn get_namespaces(&self) -> Namespaces;

    /// Returns all entity types of the given namespace
    fn get_by_namespace(&self, namespace: &str) -> EntityTypes;

    /// Returns all entity types of the given namespace
    fn get_types_by_namespace(&self, namespace: &str) -> EntityTypeIds;

    /// Returns all entity types of the given namespace
    fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> EntityTypes;

    /// Returns true, if a entity type with the given name exists.
    fn has(&self, ty: &EntityTypeId) -> bool;

    /// Returns true, if a entity type with the given fully qualified name exists.
    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool;

    /// Returns the entity type with the given name or empty.
    fn get(&self, ty: &EntityTypeId) -> Option<EntityType>;

    /// Returns the entity type with the given fully qualified name or empty.
    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<EntityType>;

    /// Returns all entity types whose names matches the given search string.
    fn find_by_type_name(&self, search: &str) -> EntityTypes;

    /// Returns the count of entity types.
    fn count(&self) -> usize;

    /// Returns the count of entity types of the given namespace.
    fn count_by_namespace(&self, namespace: &str) -> usize;

    /// Creates a new entity type.
    fn create(
        &self,
        ty: &EntityTypeId,
        description: &str,
        components: ComponentTypeIds,
        properties: PropertyTypes,
        extensions: Extensions,
    ) -> Result<EntityType, EntityTypeCreationError>;

    /// Merges the given entity_type_to_merge into an existing entity type with the same entity type id.
    fn merge(&self, entity_type_to_merge: EntityType) -> Result<EntityType, EntityTypeMergeError>;

    /// Adds the component with the given component_name to the entity type with the given name.
    fn add_component(&self, ty: &EntityTypeId, component: &ComponentTypeId) -> Result<(), EntityTypeAddComponentError>;

    // /// Adds the component with the given component_name to the entity type with the given name.
    // fn update_component(&self, entity_ty: &EntityTypeId, component_ty: &ComponentTypeId, component: &ComponentTypeId) -> Result<(), EntityTypeUpdateComponentError>;

    /// Remove the component with the given component_name from the entity type with the given name.
    fn remove_component(&self, ty: &EntityTypeId, component: &ComponentTypeId) -> Result<ComponentTypeId, EntityTypeRemoveComponentError>;

    /// Adds a property to the entity type with the given name.
    fn add_property(&self, ty: &EntityTypeId, property: PropertyType) -> Result<PropertyType, EntityTypeAddPropertyError>;

    /// Updates the property with the given property_name.
    /// It's possible to rename the property by using another name in the new property than the provided property_name.
    fn update_property(&self, ty: &EntityTypeId, property_name: &str, property_type: PropertyType) -> Result<PropertyType, EntityTypeUpdatePropertyError>;

    /// Removes the property with the given property_name from the entity type with the given name.
    fn remove_property(&self, ty: &EntityTypeId, property_name: &str) -> Result<PropertyType, EntityTypeRemovePropertyError>;

    /// Adds an extension to the entity type with the given name.
    fn add_extension(&self, ty: &EntityTypeId, extension: Extension) -> Result<ExtensionTypeId, EntityTypeAddExtensionError>;

    /// Updates the property with the given property_name.
    /// It's possible to rename the property by using another name in the new property than the provided property_name.
    fn update_extension(
        &self,
        entity_ty: &EntityTypeId,
        extension_ty: &ExtensionTypeId,
        extension: Extension,
    ) -> Result<Extension, EntityTypeUpdateExtensionError>;

    /// Removes the extension with the given extension_name from the entity type with the given name.
    fn remove_extension(&self, entity_ty: &EntityTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, EntityTypeRemoveExtensionError>;

    /// Deletes the entity type with the given name.
    fn delete(&self, ty: &EntityTypeId) -> Option<EntityType>;

    /// Validates the entity type with the given name.
    /// Tests that all components exists.
    fn validate(&self, ty: &EntityTypeId) -> bool;
}
