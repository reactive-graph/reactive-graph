use crate::PropertyType;

pub trait PropertyTypeContainer {
    /// Returns true, if the type contains an own property with the given name.
    /// Doesn't respect properties from potential components.
    fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool;

    /// Returns the own property with the given name.
    /// Doesn't respect properties from potential components.
    fn get_own_property<S: Into<String>>(&self, property_name: S) -> Option<PropertyType>;

    /// Merges the given properties into the own properties.
    fn merge_properties(&mut self, properties_to_merge: Vec<PropertyType>);
}
