use crate::Component;
use crate::ComponentTypeId;
use crate::ComponentTypeIds;
use crate::NamespacedType;

/// Reactive instance container for components.
pub trait ComponentContainer {
    /// Returns the component types of the container.
    fn get_components(&self) -> ComponentTypeIds;

    /// Adds a component to the container.
    fn add_component(&self, ty: ComponentTypeId);

    /// Adds a component to the container and initializes the reactive property instances.
    fn add_component_with_properties(&self, component: &Component);

    /// Removes a component from the container.
    fn remove_component(&self, ty: &ComponentTypeId);

    /// Returns true, if the reactive instance is composed with the given component.
    fn is_a(&self, ty: &ComponentTypeId) -> bool;

    /// Returns true, if the reactive instance is composed with all the given components.
    fn is_all(&self, tys: &ComponentTypeIds) -> bool;
}

pub trait ComponentContainerGetter {
    /// Returns the container type id.
    fn container_ty(&self) -> NamespacedType;

    /// Returns the component type id.
    fn component_ty(&self) -> ComponentTypeId;
}
