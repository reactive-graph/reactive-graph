use async_trait::async_trait;
use reactive_graph_graph::Component;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_type_system_api::ComponentExportError;
use reactive_graph_type_system_api::ComponentImportError;

#[async_trait]
pub trait ComponentImportExportManager: Send + Sync {
    /// Imports a component from a JSON file located at the given path.
    async fn import(&self, path: &str) -> Result<Component, ComponentImportError>;

    /// Exports the component with the given type id to a JSON file located at the given path.
    async fn export(&self, ty: &ComponentTypeId, path: &str) -> Result<(), ComponentExportError>;
}
