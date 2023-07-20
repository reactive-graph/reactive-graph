use crate::PropertyType;

pub trait PropertyTypeContainer {
    /// Returns true, if the type contains an own property with the given name.
    /// Doesn't respect properties from potential components.
    fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool;

    /// Returns true, if the type contains any of the given properties.
    /// Doesn't respect properties from potential components.
    fn has_any_own_properties(&self, property_names: &Vec<String>) -> bool {
        property_names.iter().any(|property_name| self.has_own_property(property_name))
    }

    /// Returns true, if the type contains all of the given properties.
    /// Doesn't respect properties from potential components.
    fn has_all_own_properties(&self, property_names: &Vec<String>) -> bool {
        property_names.iter().all(|property_name| self.has_own_property(property_name))
    }

    /// Returns the own property with the given name.
    /// Doesn't respect properties from potential components.
    fn get_own_property<S: Into<String>>(&self, property_name: S) -> Option<PropertyType>;

    /// Merges the given properties into the own properties.
    fn merge_properties(&mut self, properties_to_merge: Vec<PropertyType>);
}
