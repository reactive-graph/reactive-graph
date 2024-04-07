use async_trait::async_trait;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_type_system_api::FlowTypeExportError;
use reactive_graph_type_system_api::FlowTypeImportError;

#[async_trait]
pub trait FlowTypeImportExportManager: Send + Sync {
    /// Imports a flow type from a JSON file located at the given path.
    async fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError>;

    /// Exports the flow type with the given type id to a JSON file located at the given path.
    async fn export(&self, ty: &FlowTypeId, path: &str) -> Result<(), FlowTypeExportError>;
}
