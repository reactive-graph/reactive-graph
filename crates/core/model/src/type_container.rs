use crate::ComponentTypeId;

pub trait TypeContainer {
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
}
