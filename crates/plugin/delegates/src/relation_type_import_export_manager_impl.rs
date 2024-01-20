use std::sync::Arc;

use async_trait::async_trait;

use inexor_rgf_graph::RelationType;
use inexor_rgf_graph::RelationTypeId;
use inexor_rgf_type_system_api::RelationTypeExportError;
use inexor_rgf_type_system_api::RelationTypeImportError;

pub struct RelationTypeImportExportManagerDelegate {
    relation_type_import_export_manager: Arc<dyn inexor_rgf_type_system_api::RelationTypeImportExportManager + Send + Sync>,
}

impl RelationTypeImportExportManagerDelegate {
    pub fn new(relation_type_manager: Arc<dyn inexor_rgf_type_system_api::RelationTypeImportExportManager + Send + Sync>) -> Self {
        Self {
            relation_type_import_export_manager: relation_type_manager,
        }
    }
}

#[async_trait]
impl inexor_rgf_plugin_api::RelationTypeImportExportManager for RelationTypeImportExportManagerDelegate {
    async fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError> {
        self.relation_type_import_export_manager.import(path).await
    }

    async fn export(&self, ty: &RelationTypeId, path: &str) -> Result<(), RelationTypeExportError> {
        self.relation_type_import_export_manager.export(ty, path).await
    }
}
