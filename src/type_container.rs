pub trait TypeContainer {
    /// Returns true, if the type is a component with the given name.
    fn is_a<S: Into<String>>(&self, component_name: S) -> bool;

    /// Returns true, if the type contains an own property with the given name.
    /// Doesn't respect properties from potential components.
    fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool;

    /// Returns true, if the type contains an extension with the given name.
    fn has_own_extension<S: Into<String>>(&self, extension_name: S) -> bool;
}
