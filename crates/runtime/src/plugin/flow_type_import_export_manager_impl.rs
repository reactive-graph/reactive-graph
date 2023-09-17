use async_trait::async_trait;
use inexor_rgf_rt_api::FlowTypeExportError;
use inexor_rgf_rt_api::FlowTypeImportError;
use std::sync::Arc;

use crate::model::FlowType;
use crate::model::FlowTypeId;
use crate::plugins::FlowTypeImportExportManager;

pub struct FlowTypeImportExportManagerImpl {
    flow_type_import_export_manager: Arc<dyn crate::api::FlowTypeImportExportManager>,
}

impl FlowTypeImportExportManagerImpl {
    pub fn new(flow_type_manager: Arc<dyn crate::api::FlowTypeImportExportManager>) -> Self {
        Self {
            flow_type_import_export_manager: flow_type_manager,
        }
    }
}

#[async_trait]
impl FlowTypeImportExportManager for FlowTypeImportExportManagerImpl {
    async fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError> {
        self.flow_type_import_export_manager.import(path).await
    }

    async fn export(&self, ty: &FlowTypeId, path: &str) -> Result<(), FlowTypeExportError> {
        self.flow_type_import_export_manager.export(ty, path).await
    }
}
