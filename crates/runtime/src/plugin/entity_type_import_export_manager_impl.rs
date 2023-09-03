use std::sync::Arc;

use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::plugins::EntityTypeExportError;
use crate::plugins::EntityTypeImportError;
use crate::plugins::EntityTypeImportExportManager;

pub struct EntityTypeImportExportManagerImpl {
    entity_type_import_export_manager: Arc<dyn crate::api::EntityTypeImportExportManager>,
}

impl EntityTypeImportExportManagerImpl {
    pub fn new(entity_type_manager: Arc<dyn crate::api::EntityTypeImportExportManager>) -> Self {
        Self { entity_type_import_export_manager: entity_type_manager }
    }
}
impl EntityTypeImportExportManager for EntityTypeImportExportManagerImpl {
    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError> {
        self.entity_type_import_export_manager.import(path).map_err(|_| EntityTypeImportError {})
    }

    fn export(&self, ty: &EntityTypeId, path: &str) -> Result<(), EntityTypeExportError> {
        self.entity_type_import_export_manager.export(ty, path).map_err(|_| EntityTypeExportError {})
    }
}
