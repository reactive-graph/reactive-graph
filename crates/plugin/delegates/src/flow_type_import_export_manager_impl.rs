use std::sync::Arc;

use async_trait::async_trait;

use reactive_graph_graph::FlowType;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_type_system_api::FlowTypeExportError;
use reactive_graph_type_system_api::FlowTypeImportError;

pub struct FlowTypeImportExportManagerDelegate {
    flow_type_import_export_manager: Arc<dyn reactive_graph_type_system_api::FlowTypeImportExportManager + Send + Sync>,
}

impl FlowTypeImportExportManagerDelegate {
    pub fn new(flow_type_manager: Arc<dyn reactive_graph_type_system_api::FlowTypeImportExportManager + Send + Sync>) -> Self {
        Self {
            flow_type_import_export_manager: flow_type_manager,
        }
    }
}

#[async_trait]
impl reactive_graph_plugin_api::FlowTypeImportExportManager for FlowTypeImportExportManagerDelegate {
    async fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError> {
        self.flow_type_import_export_manager.import(path).await
    }

    async fn export(&self, ty: &FlowTypeId, path: &str) -> Result<(), FlowTypeExportError> {
        self.flow_type_import_export_manager.export(ty, path).await
    }
}
