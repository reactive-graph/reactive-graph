use crate::model::{Component, PropertyType};

#[derive(Debug)]
pub enum ComponentCreationError {
    Failed,
}

pub trait ComponentManager: Send + Sync {
    /// Returns all components
    fn get_components(&self) -> Vec<Component>;

    /// Returns true, if a component with the given name exists.
    fn has(&self, name: String) -> bool;

    /// Returns the component with the given name or empty.
    fn get(&self, name: String) -> Option<Component>;

    /// Creates a new component with the given name and the given properties.
    fn create(&self, name: String, properties: Vec<PropertyType>);

    /// Deletes the component with the given name.
    fn delete(&self, name: String);
}
