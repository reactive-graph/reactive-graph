use crate::ComponentTypeId;
use crate::ComponentTypeIds;
use crate::Components;

pub trait ComponentTypeIdContainer {
    /// Returns true, if the type is composed with a component of the given type.
    fn is_a(&self, ty: &ComponentTypeId) -> bool;

    /// Returns true, if the type is composed with any of the given components.
    fn is_any(&self, component_tys: &ComponentTypeIds) -> bool {
        component_tys.iter().any(|ty| self.is_a(&ty))
    }

    /// Returns true, if the type is composed with every given components.
    fn is_all(&self, component_tys: &ComponentTypeIds) -> bool {
        component_tys.iter().all(|ty| self.is_a(&ty))
    }

    /// Adds the given component.
    fn add_component<C: Into<ComponentTypeId>>(&self, ty: C) -> bool;

    /// Adds the given components.
    fn add_components<C: Into<ComponentTypeIds>>(&mut self, components_to_add: C);

    /// Removes the component with the given type id.
    fn remove_component(&self, ty: &ComponentTypeId) -> Option<ComponentTypeId>;

    /// Removes the given components.
    fn remove_components<C: Into<ComponentTypeIds>>(&mut self, components_to_remove: C);

    fn get_components_cloned(&self) -> ComponentTypeIds;
}

pub trait NamespacedTypeComponentTypeIdContainer<T, AddComponentError, RemoveComponentError> {
    /// Returns all types having the specified component.
    fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> Self;

    /// Adds the component with the given component_name to the entity type with the given name.
    fn add_component(&self, ty: &T, component_ty: &ComponentTypeId) -> Result<(), AddComponentError>;

    /// Remove the component with the given component_name from the entity type with the given name.
    fn remove_component(&self, ty: &T, component_ty: &ComponentTypeId) -> Result<ComponentTypeId, RemoveComponentError>;
}

pub trait NamespacedTypeComponentPropertiesContainer<T, MergeComponentPropertiesError> {
    /// Resolves properties from the given components and merges them into the .
    fn merge_component_properties<C: Into<Components>>(&self, components: C) -> Result<(), MergeComponentPropertiesError>;
}
