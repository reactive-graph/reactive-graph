use indradb::Identifier;

use crate::PropertyType;

pub trait TypeContainer {
    /// Returns the fully qualified name of the type.
    ///
    /// The fully qualified name contains the namespace and the name.
    fn fully_qualified_name(&self) -> String;

    /// Returns true, if the type is a component with the given name.
    fn is_a<S: Into<String>>(&self, component_name: S) -> bool;

    /// Returns true, if the type contains an own property with the given name.
    /// Doesn't respect properties from potential components.
    fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool;

    /// Returns the own property with the given name.
    /// Doesn't respect properties from potential components.
    fn get_own_property<S: Into<String>>(&self, property_name: S) -> Option<PropertyType>;
}
