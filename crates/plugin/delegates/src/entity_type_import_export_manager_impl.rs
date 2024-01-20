use async_trait::async_trait;
use std::sync::Arc;

use inexor_rgf_graph::EntityType;
use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_type_system_api::EntityTypeExportError;
use inexor_rgf_type_system_api::EntityTypeImportError;

pub struct EntityTypeImportExportManagerDelegate {
    entity_type_import_export_manager: Arc<dyn inexor_rgf_type_system_api::EntityTypeImportExportManager + Send + Sync>,
}

impl EntityTypeImportExportManagerDelegate {
    pub fn new(entity_type_manager: Arc<dyn inexor_rgf_type_system_api::EntityTypeImportExportManager + Send + Sync>) -> Self {
        Self {
            entity_type_import_export_manager: entity_type_manager,
        }
    }
}

#[async_trait]
impl inexor_rgf_plugin_api::EntityTypeImportExportManager for EntityTypeImportExportManagerDelegate {
    async fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError> {
        self.entity_type_import_export_manager.import(path).await
    }

    async fn export(&self, ty: &EntityTypeId, path: &str) -> Result<(), EntityTypeExportError> {
        self.entity_type_import_export_manager.export(ty, path).await
    }
}
