use crate::model::Component;
use crate::model::ComponentTypeId;
use crate::rt_api::ComponentExportError;
use crate::rt_api::ComponentImportError;
use async_trait::async_trait;

#[async_trait]
pub trait ComponentImportExportManager: Send + Sync {
    /// Imports a component from a JSON file located at the given path.
    async fn import(&self, path: &str) -> Result<Component, ComponentImportError>;

    /// Exports the component with the given type id to a JSON file located at the given path.
    async fn export(&self, ty: &ComponentTypeId, path: &str) -> Result<(), ComponentExportError>;
}
