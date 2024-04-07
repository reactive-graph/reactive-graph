use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_graph::Component;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_lifecycle::Lifecycle;

use crate::ComponentExportError;
use crate::ComponentImportError;

#[injectable]
#[async_trait]
pub trait ComponentImportExportManager: Send + Sync + Lifecycle {
    /// Imports a component from a JSON file located at the given path.
    async fn import(&self, path: &str) -> Result<Component, ComponentImportError>;

    /// Exports the component with the given name to a JSON file located at the given path.
    async fn export(&self, ty: &ComponentTypeId, path: &str) -> Result<(), ComponentExportError>;
}
