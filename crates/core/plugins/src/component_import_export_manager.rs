use crate::model::Component;
use crate::model::ComponentTypeId;

#[derive(Debug)]
pub struct ComponentImportError;

#[derive(Debug)]
pub struct ComponentExportError;

pub trait ComponentImportExportManager: Send + Sync {
    /// Imports a component from a JSON file located at the given path.
    fn import(&self, path: &str) -> Result<Component, ComponentImportError>;

    /// Exports the component with the given type id to a JSON file located at the given path.
    fn export(&self, ty: &ComponentTypeId, path: &str) -> Result<(), ComponentExportError>;
}
