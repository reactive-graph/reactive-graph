use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::Component;
use crate::model::Extension;
use crate::model::PropertyType;
use crate::plugins::ComponentProvider;

#[derive(Debug)]
pub enum ComponentPropertyError {
    PropertyAlreadyExists,
}

#[derive(Debug)]
pub enum ComponentExtensionError {
    ExtensionAlreadyExists,
}

#[async_trait]
pub trait ComponentManager: Send + Sync + Lifecycle {
    /// Registers the given component
    fn register(&self, component: Component);

    /// Returns all components
    fn get_components(&self) -> Vec<Component>;

    /// Returns all components of the given namespace
    fn get_components_by_namespace(&self, namespace: &str) -> Vec<Component>;

    /// Returns true, if a component with the given name exists.
    fn has(&self, name: &str) -> bool;

    /// Returns the component with the given name or empty.
    fn get(&self, name: &str) -> Option<Component>;

    /// Returns all components whose names matches the given search string.
    fn find(&self, search: &str) -> Vec<Component>;

    /// Returns the count of components.
    fn count(&self) -> usize;

    /// Creates a new component with the given namespace, name, description, properties and extensions.
    fn create(&self, namespace: &str, name: &str, description: &str, properties: Vec<PropertyType>, extensions: Vec<Extension>);

    /// Replaces the component with the given name with the given component.
    fn replace(&self, name: &str, component: Component);

    /// Adds a property to the component with the given name.
    fn add_property(&self, name: &str, property: PropertyType) -> Result<(), ComponentPropertyError>;

    /// Removes the property with the given property_name from the component with the given name.
    fn remove_property(&self, name: &str, property_name: &str);

    /// Adds an extension to the component with the given name.
    fn add_extension(&self, name: &str, extension: Extension) -> Result<(), ComponentExtensionError>;

    /// Removes the extension with the given extension_name from the component with the given name.
    fn remove_extension(&self, name: &str, extension_name: &str);

    /// Deletes the component with the given name.
    fn delete(&self, name: &str);

    /// Imports a component from a JSON file located at the given path.
    fn import(&self, path: &str);

    /// Exports the component with the given name to a JSON file located at the given path.
    fn export(&self, name: &str, path: &str);

    /// Registers a component provider.
    fn add_provider(&self, component_provider: Arc<dyn ComponentProvider>);

    /// Returns the list of component categories
    fn get_component_categories(&self) -> Vec<String>;
}
