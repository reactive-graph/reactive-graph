use crate::ComponentTypeId;
use crate::PropertyType;

pub trait TypeContainer {
    /// Returns true, if the type is composed with a component of the given type.
    fn is_a(&self, ty: &ComponentTypeId) -> bool;

    /// Returns true, if the type contains an own property with the given name.
    /// Doesn't respect properties from potential components.
    fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool;

    /// Returns the own property with the given name.
    /// Doesn't respect properties from potential components.
    fn get_own_property<S: Into<String>>(&self, property_name: S) -> Option<PropertyType>;
}
