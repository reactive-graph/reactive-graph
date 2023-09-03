use std::sync::Arc;

use crate::model::Component;
use crate::model::ComponentTypeId;
use crate::plugins::ComponentExportError;
use crate::plugins::ComponentImportError;
use crate::plugins::ComponentImportExportManager;

pub struct ComponentImportExportManagerImpl {
    component_import_export_manager: Arc<dyn crate::api::ComponentImportExportManager>,
}

impl ComponentImportExportManagerImpl {
    pub fn new(component_manager: Arc<dyn crate::api::ComponentImportExportManager>) -> Self {
        Self { component_import_export_manager: component_manager }
    }
}
impl ComponentImportExportManager for ComponentImportExportManagerImpl {
    fn import(&self, path: &str) -> Result<Component, ComponentImportError> {
        self.component_import_export_manager.import(path).map_err(|_| ComponentImportError {})
    }

    fn export(&self, ty: &ComponentTypeId, path: &str) -> Result<(), ComponentExportError> {
        self.component_import_export_manager.export(ty, path).map_err(|_| ComponentExportError {})
    }
}
