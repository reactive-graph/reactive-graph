use async_trait::async_trait;

use crate::error::types::component::ComponentImportError;
use crate::error::types::component::ComponentExportError;
use crate::model::Component;
use crate::model::ComponentTypeId;

#[async_trait]
pub trait ComponentImportExportManager: Send + Sync {
    /// Imports a component from a JSON file located at the given path.
    fn import(&self, path: &str) -> Result<Component, ComponentImportError>;

    /// Exports the component with the given name to a JSON file located at the given path.
    fn export(&self, ty: &ComponentTypeId, path: &str) -> Result<(), ComponentExportError>;
}
