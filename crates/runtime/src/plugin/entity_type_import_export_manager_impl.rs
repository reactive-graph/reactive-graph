use std::sync::Arc;

use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::plugins::EntityTypeImportExportManager;
use crate::rt_api::EntityTypeExportError;
use crate::rt_api::EntityTypeImportError;

pub struct EntityTypeImportExportManagerImpl {
    entity_type_import_export_manager: Arc<dyn crate::api::EntityTypeImportExportManager>,
}

impl EntityTypeImportExportManagerImpl {
    pub fn new(entity_type_manager: Arc<dyn crate::api::EntityTypeImportExportManager>) -> Self {
        Self {
            entity_type_import_export_manager: entity_type_manager,
        }
    }
}
impl EntityTypeImportExportManager for EntityTypeImportExportManagerImpl {
    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError> {
        self.entity_type_import_export_manager.import(path)
    }

    fn export(&self, ty: &EntityTypeId, path: &str) -> Result<(), EntityTypeExportError> {
        self.entity_type_import_export_manager.export(ty, path)
    }
}
