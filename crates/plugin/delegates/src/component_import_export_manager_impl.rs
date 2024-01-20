use async_trait::async_trait;
use std::sync::Arc;

use inexor_rgf_graph::Component;
use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_type_system_api::ComponentExportError;
use inexor_rgf_type_system_api::ComponentImportError;

pub struct ComponentImportExportManagerDelegate {
    component_import_export_manager: Arc<dyn inexor_rgf_type_system_api::ComponentImportExportManager + Send + Sync>,
}

impl ComponentImportExportManagerDelegate {
    pub fn new(component_manager: Arc<dyn inexor_rgf_type_system_api::ComponentImportExportManager + Send + Sync>) -> Self {
        Self {
            component_import_export_manager: component_manager,
        }
    }
}

#[async_trait]
impl inexor_rgf_plugin_api::ComponentImportExportManager for ComponentImportExportManagerDelegate {
    async fn import(&self, path: &str) -> Result<Component, ComponentImportError> {
        self.component_import_export_manager.import(path).await
    }

    async fn export(&self, ty: &ComponentTypeId, path: &str) -> Result<(), ComponentExportError> {
        self.component_import_export_manager.export(ty, path).await
    }
}
