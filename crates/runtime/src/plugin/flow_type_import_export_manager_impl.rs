use std::sync::Arc;

use crate::model::FlowType;
use crate::model::FlowTypeId;
use crate::plugins::FlowTypeExportError;
use crate::plugins::FlowTypeImportError;
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
impl FlowTypeImportExportManager for FlowTypeImportExportManagerImpl {
    fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError> {
        self.flow_type_import_export_manager.import(path).map_err(|_| FlowTypeImportError {})
    }

    fn export(&self, ty: &FlowTypeId, path: &str) -> Result<(), FlowTypeExportError> {
        self.flow_type_import_export_manager.export(ty, path).map_err(|_| FlowTypeExportError {})
    }
}
