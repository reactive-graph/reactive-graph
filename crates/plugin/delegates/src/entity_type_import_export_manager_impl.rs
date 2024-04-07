use async_trait::async_trait;
use std::sync::Arc;

use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_type_system_api::EntityTypeExportError;
use reactive_graph_type_system_api::EntityTypeImportError;

pub struct EntityTypeImportExportManagerDelegate {
    entity_type_import_export_manager: Arc<dyn reactive_graph_type_system_api::EntityTypeImportExportManager + Send + Sync>,
}

impl EntityTypeImportExportManagerDelegate {
    pub fn new(entity_type_manager: Arc<dyn reactive_graph_type_system_api::EntityTypeImportExportManager + Send + Sync>) -> Self {
        Self {
            entity_type_import_export_manager: entity_type_manager,
        }
    }
}

#[async_trait]
impl reactive_graph_plugin_api::EntityTypeImportExportManager for EntityTypeImportExportManagerDelegate {
    async fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError> {
        self.entity_type_import_export_manager.import(path).await
    }

    async fn export(&self, ty: &EntityTypeId, path: &str) -> Result<(), EntityTypeExportError> {
        self.entity_type_import_export_manager.export(ty, path).await
    }
}
