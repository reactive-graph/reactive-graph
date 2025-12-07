use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypeAddComponentError;
use reactive_graph_graph::EntityTypeAddExtensionError;
use reactive_graph_graph::EntityTypeAddPropertyError;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::EntityTypeRemoveComponentError;
use reactive_graph_graph::EntityTypeRemoveExtensionError;
use reactive_graph_graph::EntityTypeRemovePropertyError;
use reactive_graph_graph::EntityTypeUpdateError;
use reactive_graph_graph::EntityTypes;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_type_system_api::EntityTypeCreationError;

pub trait EntityTypeManager: Send + Sync {
    /// Returns all entity types.
    fn get_all(&self) -> EntityTypes;

    /// Returns all entity types of the given namespace.
    fn get_by_namespace(&self, namespace: &Namespace) -> EntityTypes;

    /// Returns true, if a entity type with the given name exists.
    fn has(&self, ty: &EntityTypeId) -> bool;

    /// Returns the entity type with the given name or empty.
    fn get(&self, ty: &EntityTypeId) -> Option<EntityType>;

    /// Returns all entity types whose names matches the given search string.
    fn find(&self, search: &str) -> EntityTypes;

    /// Returns the count of entity types.
    fn count(&self) -> usize;

    /// Returns the count of entity types of the given namespace.
    fn count_by_namespace(&self, namespace: &Namespace) -> usize;

    /// Creates a new entity type.
    fn create(
        &self,
        ty: &EntityTypeId,
        description: &str,
        components: ComponentTypeIds,
        properties: PropertyTypes,
        extensions: Extensions,
    ) -> Result<EntityType, EntityTypeCreationError>;

    /// Updates the description of the given entity type.
    fn update_description(&self, ty: &EntityTypeId, description: &str) -> Result<EntityType, EntityTypeUpdateError>;

    /// Adds the component with the given component_name to the given entity type.
    fn add_component(&self, ty: &EntityTypeId, component: &ComponentTypeId) -> Result<(), EntityTypeAddComponentError>;

    /// Remove the component with the given component_name from the given entity type.
    fn remove_component(&self, ty: &EntityTypeId, component: &ComponentTypeId) -> Result<ComponentTypeId, EntityTypeRemoveComponentError>;

    /// Adds a property to the given entity type.
    fn add_property(&self, ty: &EntityTypeId, property: PropertyType) -> Result<PropertyType, EntityTypeAddPropertyError>;

    /// Removes the property with the given property_name from the given entity type.
    fn remove_property(&self, ty: &EntityTypeId, property_name: &str) -> Result<PropertyType, EntityTypeRemovePropertyError>;

    /// Adds an extension to the given entity type.
    fn add_extension(&self, ty: &EntityTypeId, extension: Extension) -> Result<ExtensionTypeId, EntityTypeAddExtensionError>;

    /// Removes the extension with the given type from the given entity type.
    fn remove_extension(&self, entity_ty: &EntityTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, EntityTypeRemoveExtensionError>;

    /// Deletes the entity type.
    fn delete(&self, ty: &EntityTypeId) -> Option<EntityType>;

    /// Validates the entity type with the given name.
    /// Tests that all components exists.
    fn validate(&self, ty: &EntityTypeId) -> bool;
}
