use std::collections::HashSet;
use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::Component;
use crate::model::ComponentTypeId;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::PropertyType;
use crate::plugins::ComponentProvider;

#[derive(Debug)]
pub enum ComponentRegistrationError {
    ComponentAlreadyExists(ComponentTypeId),
}

#[derive(Debug)]
pub enum ComponentMergeError {
    ComponentDoesNotExist(ComponentTypeId),
}

#[derive(Debug)]
pub enum ComponentCreationError {
    RegistrationError(ComponentRegistrationError),
}

#[derive(Debug)]
pub enum ComponentImportError {
    Io(std::io::Error),
    Deserialization(serde_json::Error),
    RegistrationError(ComponentRegistrationError),
}

#[derive(Debug)]
pub enum ComponentPropertyError {
    PropertyAlreadyExists,
}

#[derive(Debug)]
pub enum ComponentPropertyUpdateError {
    PropertyDoesNotExist,
}

#[derive(Debug)]
pub enum ComponentExtensionError {
    ExtensionAlreadyExists,
}

#[derive(Debug)]
pub enum ComponentExtensionUpdateError {
    ExtensionDoesNotExist,
}

impl From<std::io::Error> for ComponentImportError {
    fn from(e: std::io::Error) -> Self {
        ComponentImportError::Io(e)
    }
}

impl From<serde_json::Error> for ComponentImportError {
    fn from(e: serde_json::Error) -> Self {
        ComponentImportError::Deserialization(e)
    }
}

#[async_trait]
pub trait ComponentManager: Send + Sync + Lifecycle {
    /// Registers the given component
    fn register(&self, component: Component) -> Result<Component, ComponentRegistrationError>;

    /// Returns all components
    fn get_all(&self) -> Vec<Component>;

    /// Returns all defined namespaces.
    fn get_namespaces(&self) -> HashSet<String>;

    /// Returns all components of the given namespace
    fn get_by_namespace(&self, namespace: &str) -> Vec<Component>;

    /// Returns true, if a component with the given type exists.
    fn has(&self, ty: &ComponentTypeId) -> bool;

    /// Returns true, if a component with the given fully qualified name exists.
    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool;

    /// Returns the component with the given name or empty.
    fn get(&self, ty: &ComponentTypeId) -> Option<Component>;

    /// Returns the component with the given fully qualified name or empty.
    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<Component>;

    /// Returns all components whose type names matches the given search string.
    fn find(&self, search: &str) -> Vec<Component>;

    /// Returns the count of components.
    fn count(&self) -> usize;

    /// Returns the count of components of the given namespace.
    fn count_by_namespace(&self, namespace: &str) -> usize;

    /// Creates a new component of the given type with the description, properties and extensions.
    fn create(
        &self,
        ty: &ComponentTypeId,
        description: &str,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<Component, ComponentCreationError>;

    /// Replaces the component with the given name with the given component.
    fn replace(&self, ty: &ComponentTypeId, component: Component);

    /// Merges the given component_to_merge into an existing component with the same component type id.
    fn merge(&self, component_to_merge: Component) -> Result<Component, ComponentMergeError>;

    /// Adds a property to the given component.
    fn add_property(&self, ty: &ComponentTypeId, property: PropertyType) -> Result<(), ComponentPropertyError>;

    /// Updates the property with the given property_name.
    /// It's possible to rename the property by using another name in the new property than the provided property_name.
    fn update_property(&self, ty: &ComponentTypeId, property_name: &str, property: PropertyType) -> Result<(), ComponentPropertyUpdateError>;

    /// Removes the property with the given property_name from the given component.
    fn remove_property(&self, ty: &ComponentTypeId, property_name: &str);

    /// Adds an extension to the given component.
    fn add_extension(&self, ty: &ComponentTypeId, extension: Extension) -> Result<(), ComponentExtensionError>;

    /// Replaces the extension of the given component.
    fn update_extension(
        &self,
        component_ty: &ComponentTypeId,
        extension_ty: &ExtensionTypeId,
        extension: Extension,
    ) -> Result<(), ComponentExtensionUpdateError>;

    /// Removes the extension with the given extension_name from the given component.
    fn remove_extension(&self, component_ty: &ComponentTypeId, extension_ty: &ExtensionTypeId);

    /// Deletes the component with the given name.
    fn delete(&self, ty: &ComponentTypeId) -> bool;

    /// Imports a component from a JSON file located at the given path.
    fn import(&self, path: &str) -> Result<Component, ComponentImportError>;

    /// Exports the component with the given name to a JSON file located at the given path.
    fn export(&self, ty: &ComponentTypeId, path: &str);

    /// Registers a component provider.
    fn add_provider(&self, component_provider: Arc<dyn ComponentProvider>);
}
