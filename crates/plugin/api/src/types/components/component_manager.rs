use reactive_graph_graph::Component;
use reactive_graph_graph::ComponentAddExtensionError;
use reactive_graph_graph::ComponentAddPropertyError;
use reactive_graph_graph::ComponentRemoveExtensionError;
use reactive_graph_graph::ComponentRemovePropertyError;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentUpdateExtensionError;
use reactive_graph_graph::ComponentUpdatePropertyError;
use reactive_graph_graph::Components;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_type_system_api::ComponentCreationError;

pub trait ComponentManager: Send + Sync {
    /// Returns all components
    fn get_all(&self) -> Components;

    /// Returns all components of the given namespace.
    fn get_by_namespace(&self, namespace: &str) -> Components;

    /// Returns true, if a component with the given name exists.
    fn has(&self, ty: &ComponentTypeId) -> bool;

    /// Returns true, if a component with the given fully qualified name exists.
    fn has_by_type(&self, namespace: &str, name: &str) -> bool;

    /// Returns the component with the given name or empty.
    fn get(&self, ty: &ComponentTypeId) -> Option<Component>;

    /// Returns the component with the given fully qualified name or empty.
    fn get_by_type(&self, namespace: &str, name: &str) -> Option<Component>;

    /// Returns all components whose names matches the given search string.
    fn find_by_type_name(&self, search: &str) -> Components;

    /// Returns the count of components.
    fn count(&self) -> usize;

    /// Returns the count of components of the given namespace.
    fn count_by_namespace(&self, namespace: &str) -> usize;

    /// Creates a new component with the given name and the given properties.
    fn create(&self, ty: &ComponentTypeId, description: &str, properties: PropertyTypes, extensions: Extensions) -> Result<Component, ComponentCreationError>;

    /// Replaces the component with the given name with the given component.
    fn replace(&self, ty: &ComponentTypeId, component: Component);

    /// Adds a property to the component with the given name.
    fn add_property(&self, ty: &ComponentTypeId, property: PropertyType) -> Result<PropertyType, ComponentAddPropertyError>;

    /// Adds a property to the component with the given name.
    fn update_property(&self, ty: &ComponentTypeId, property_name: &str, property: PropertyType) -> Result<PropertyType, ComponentUpdatePropertyError>;

    /// Removes the property with the given property_name from the component with the given name.
    fn remove_property(&self, ty: &ComponentTypeId, property_name: &str) -> Result<PropertyType, ComponentRemovePropertyError>;

    /// Adds an extension to the component with the given name.
    fn add_extension(&self, ty: &ComponentTypeId, extension: Extension) -> Result<ExtensionTypeId, ComponentAddExtensionError>;

    /// Replaces the extension of the given component.
    fn update_extension(
        &self,
        component_ty: &ComponentTypeId,
        extension_ty: &ExtensionTypeId,
        extension: Extension,
    ) -> Result<Extension, ComponentUpdateExtensionError>;

    /// Removes the extension with the given type from the component with the given type.
    fn remove_extension(&self, component_ty: &ComponentTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, ComponentRemoveExtensionError>;

    /// Deletes the component with the given name.
    fn delete(&self, ty: &ComponentTypeId) -> bool;
}
