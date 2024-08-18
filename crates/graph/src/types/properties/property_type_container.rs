use crate::AddPropertyError;
use crate::PropertyType;
use crate::PropertyTypes;
use crate::RemovePropertyError;
use crate::UpdatePropertyError;

/// A type which contains property types.
pub trait PropertyTypeContainer {
    /// Returns true, if the type contains an own property with the given name.
    /// Doesn't respect properties from potential components.
    fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool;

    /// Returns true, if the type contains any of the given properties.
    /// Doesn't respect properties from potential components.
    fn has_any_own_properties(&self, property_names: &[String]) -> bool {
        property_names.iter().any(|property_name| self.has_own_property(property_name))
    }

    /// Returns true, if the type contains all of the given properties.
    /// Doesn't respect properties from potential components.
    fn has_all_own_properties(&self, property_names: &[String]) -> bool {
        property_names.iter().all(|property_name| self.has_own_property(property_name))
    }

    /// Returns the own property with the given name.
    /// Doesn't respect properties from potential components.
    fn get_own_property<S: Into<String>>(&self, property_name: S) -> Option<PropertyType>;

    /// Adds the given property.
    fn add_property<S: Into<PropertyType>>(&self, property_type: S) -> Result<PropertyType, AddPropertyError>;

    /// Updates the property with the given property_name.
    /// It's possible to rename the property by using another name in the new property than the provided property_name.
    fn update_property<N: Into<String>, S: Into<PropertyType>>(&self, property_name: N, property_type: S) -> Result<PropertyType, UpdatePropertyError>;

    /// Removes the property with the given name.
    fn remove_property<S: Into<String>>(&self, property_name: S) -> Result<PropertyType, RemovePropertyError>;

    /// Merges the given properties into the own properties.
    fn merge_properties<P: Into<PropertyTypes>>(&mut self, properties_to_merge: P);
}

/// Collection of a type which contains property types.
pub trait NamespacedTypePropertyTypeContainer<T, AddPropertyError, UpdatePropertyError, RemovePropertyError, MergePropertiesError> {
    /// Adds a property to the given type.
    fn add_property<P: Into<PropertyType>>(&self, ty: &T, property_type: P) -> Result<PropertyType, AddPropertyError>;

    /// Updates the property with the given name of the given type.
    /// It's possible to rename the property by using another name in the new property than the provided property_name.
    fn update_property<N: Into<String>, P: Into<PropertyType>>(&self, ty: &T, property_name: N, property_type: P) -> Result<PropertyType, UpdatePropertyError>;

    /// Remove the property with the given name from the given type.
    fn remove_property<N: Into<String>>(&self, ty: &T, property_name: N) -> Result<PropertyType, RemovePropertyError>;

    /// Merges the given properties into the properties of the given type.
    fn merge_properties<P: Into<PropertyTypes>>(&self, ty: &T, properties_to_merge: P) -> Result<(), MergePropertiesError>;
}
