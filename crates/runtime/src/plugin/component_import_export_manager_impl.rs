use async_trait::async_trait;
use std::sync::Arc;

use crate::model::Component;
use crate::model::ComponentTypeId;
use crate::plugins::ComponentImportExportManager;
use crate::rt_api::ComponentExportError;
use crate::rt_api::ComponentImportError;

pub struct ComponentImportExportManagerImpl {
    component_import_export_manager: Arc<dyn crate::api::ComponentImportExportManager>,
}

impl ComponentImportExportManagerImpl {
    pub fn new(component_manager: Arc<dyn crate::api::ComponentImportExportManager>) -> Self {
        Self {
            component_import_export_manager: component_manager,
        }
    }
}

#[async_trait]
impl ComponentImportExportManager for ComponentImportExportManagerImpl {
    async fn import(&self, path: &str) -> Result<Component, ComponentImportError> {
        self.component_import_export_manager.import(path).await
    }

    async fn export(&self, ty: &ComponentTypeId, path: &str) -> Result<(), ComponentExportError> {
        self.component_import_export_manager.export(ty, path).await
    }
}
