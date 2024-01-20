use async_trait::async_trait;
use springtime_di::injectable;

use crate::ComponentCreationError;
use crate::ComponentRegistrationError;
use inexor_rgf_graph::Component;
use inexor_rgf_graph::ComponentAddExtensionError;
use inexor_rgf_graph::ComponentAddPropertyError;
use inexor_rgf_graph::ComponentMergeError;
use inexor_rgf_graph::ComponentRemoveExtensionError;
use inexor_rgf_graph::ComponentRemovePropertyError;
use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_graph::ComponentTypeIds;
use inexor_rgf_graph::ComponentUpdateExtensionError;
use inexor_rgf_graph::ComponentUpdatePropertyError;
use inexor_rgf_graph::Components;
use inexor_rgf_graph::Extension;
use inexor_rgf_graph::ExtensionTypeId;
use inexor_rgf_graph::Namespaces;
use inexor_rgf_graph::PropertyType;
use inexor_rgf_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait ComponentManager: Send + Sync + Lifecycle {
    /// Registers the given component type.
    fn register(&self, component: Component) -> Result<Component, ComponentRegistrationError>;

    /// Returns all component types.
    fn get_all(&self) -> Components;

    /// Returns the ids of all component types.
    fn get_type_ids(&self) -> ComponentTypeIds;

    /// Returns all defined namespaces.
    fn get_namespaces(&self) -> Namespaces;

    /// Returns all components of the given namespace.
    fn get_by_namespace(&self, namespace: &str) -> Components;

    /// Returns all component types of the given namespace.
    fn get_types_by_namespace(&self, namespace: &str) -> ComponentTypeIds;

    /// Returns true, if a component with the given type exists.
    fn has(&self, ty: &ComponentTypeId) -> bool;

    /// Returns true, if a component with the given fully qualified name exists.
    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool;

    /// Returns the component with the given name or empty.
    fn get(&self, ty: &ComponentTypeId) -> Option<Component>;

    /// Returns the component with the given fully qualified name or empty.
    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<Component>;

    /// Returns the components with the given types.
    fn get_by_types(&self, tys: ComponentTypeIds) -> Components;

    /// Returns all components whose type names matches the given search string.
    fn find_by_type_name(&self, search: &str) -> Components;

    /// Returns the count of components.
    fn count(&self) -> usize;

    /// Returns the count of components of the given namespace.
    fn count_by_namespace(&self, namespace: &str) -> usize;

    /// Creates a new component of the given type with the description, properties and extensions.
    fn create_component(
        &self,
        ty: &ComponentTypeId,
        description: &str,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<Component, ComponentCreationError>;

    /// Replaces the component with the given name with the given component.
    fn replace(&self, ty: &ComponentTypeId, component: Component);

    /// Merges the given component_to_merge into an existing component with the same component type id.
    fn merge(&self, component_to_merge: Component) -> Result<Component, ComponentMergeError>;

    /// Adds a property to the given component.
    fn add_property(&self, ty: &ComponentTypeId, property: PropertyType) -> Result<PropertyType, ComponentAddPropertyError>;

    /// Updates the property with the given property_name.
    /// It's possible to rename the property by using another name in the new property than the provided property_name.
    fn update_property(&self, ty: &ComponentTypeId, property_name: &str, property: PropertyType) -> Result<PropertyType, ComponentUpdatePropertyError>;

    /// Removes the property with the given property_name from the given component.
    fn remove_property(&self, ty: &ComponentTypeId, property_name: &str) -> Result<PropertyType, ComponentRemovePropertyError>;

    /// Adds an extension to the given component.
    fn add_extension(&self, ty: &ComponentTypeId, extension: Extension) -> Result<ExtensionTypeId, ComponentAddExtensionError>;

    /// Replaces the extension of the given component.
    fn update_extension(
        &self,
        component_ty: &ComponentTypeId,
        extension_ty: &ExtensionTypeId,
        extension: Extension,
    ) -> Result<Extension, ComponentUpdateExtensionError>;

    /// Removes the extension with the given extension_name from the given component.
    fn remove_extension(&self, component_ty: &ComponentTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, ComponentRemoveExtensionError>;

    /// Deletes the component with the given name.
    fn delete(&self, ty: &ComponentTypeId) -> bool;
}
