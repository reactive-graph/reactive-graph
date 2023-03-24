use crate::ComponentTypeId;

pub trait TypeContainer {
    /// Returns true, if the type is composed with a component of the given type.
    fn is_a(&self, ty: &ComponentTypeId) -> bool;
}
