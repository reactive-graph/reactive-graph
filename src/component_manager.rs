use crate::model::Component;
use crate::model::Extension;
use crate::model::PropertyType;

#[derive(Debug)]
pub enum ComponentManagerError {
    InitializationError,
}

#[derive(Debug)]
pub enum ComponentCreationError {
    Failed,
}

#[derive(Debug)]
pub enum ComponentImportError {
    Failed,
}

pub trait ComponentManager: Send + Sync {
    /// Returns all components
    fn get_components(&self) -> Vec<Component>;

    /// Returns all components of the given namespace.
    fn get_components_by_namespace(&self, namespace: &str) -> Vec<Component>;

    /// Returns true, if a component with the given name exists.
    fn has(&self, name: &str) -> bool;

    /// Returns the component with the given name or empty.
    fn get(&self, name: &str) -> Option<Component>;

    /// Returns all components whose names matches the given search string.
    fn find(&self, search: &str) -> Vec<Component>;

    /// Returns the count of components.
    fn count(&self) -> usize;

    /// Creates a new component with the given name and the given properties.
    fn create(
        &self,
        namespace: &str,
        name: &str,
        description: &str,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<Component, ComponentCreationError>;

    /// Replaces the component with the given name with the given component.
    fn replace(&self, name: &str, component: Component);

    /// Adds a property to the component with the given name.
    fn add_property(&self, name: &str, property: PropertyType);

    /// Removes the property with the given property_name from the component with the given name.
    fn remove_property(&self, name: &str, property_name: &str);

    /// Adds an extension to the component with the given name.
    fn add_extension(&self, name: &str, extension: Extension);

    /// Removes the extension with the given extension_name from the component with the given name.
    fn remove_extension(&self, name: &str, extension_name: &str);

    /// Deletes the component with the given name.
    fn delete(&self, name: &str);

    /// Imports a component from a JSON file located at the given path.
    fn import(&self, path: &str) -> Result<Component, ComponentImportError>;

    /// Exports the component with the given name to a JSON file located at the given path.
    fn export(&self, name: &str, path: &str);
}
