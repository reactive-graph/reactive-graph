use crate::ComponentTypeId;
use crate::ComponentTypeIds;

pub trait ComponentTypeIdContainer {
    /// Returns true, if the type is composed with a component of the given type.
    fn is_a(&self, ty: &ComponentTypeId) -> bool;

    /// Returns true, if the type is composed with any of the given components.
    fn is_any(&self, component_tys: &Vec<ComponentTypeId>) -> bool {
        component_tys.iter().any(|ty| self.is_a(ty))
    }

    /// Returns true, if the type is composed with every given components.
    fn is_all(&self, component_tys: &Vec<ComponentTypeId>) -> bool {
        component_tys.iter().all(|ty| self.is_a(ty))
    }

    /// Adds the given component.
    fn add_component<C: Into<ComponentTypeId>>(&self, ty: C) -> bool;

    /// Adds the given components.
    fn add_components <C: Into<ComponentTypeIds>> (&mut self, components_to_add: C);

    /// Removes the component with the given type id.
    fn remove_component(&self, ty: &ComponentTypeId) -> Option<ComponentTypeId>;

    /// Removes the given components.
    fn remove_components <C: Into<ComponentTypeIds>> (&mut self, components_to_remove: C);
}

pub trait NamespacedTypeComponentTypeIdContainer<T, AddComponentError, RemoveComponentError> {
    /// Returns all types having the specified component.
    fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> Self;

    /// Adds the component with the given component_name to the entity type with the given name.
    fn add_component(&self, ty: &T, component_ty: &ComponentTypeId) -> Result<(), AddComponentError>;

    /// Remove the component with the given component_name from the entity type with the given name.
    fn remove_component(&self, ty: &T, component_ty: &ComponentTypeId) -> Result<ComponentTypeId, RemoveComponentError>;
}

// pub trait ComponentTypeIdContainerContainer
//     where Self: IntoIterator<Self::Item=>
// {
//     type ID;
//     type Item;
//
//     fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> Self  {
//         self.0.iter()
//             .filter(|entity_type| entity_type.is_a(component_ty))
//             .map(|entity_type| entity_type.value().clone())
//             .collect()
//     }
//
// }
//
// impl IntoIterator for EntityTypes {
//     type Item = (EntityTypeId, EntityType);
//     type IntoIter = OwningIter<EntityTypeId, EntityType>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }
