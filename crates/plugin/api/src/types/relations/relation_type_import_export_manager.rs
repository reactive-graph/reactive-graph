use async_trait::async_trait;
use inexor_rgf_graph::RelationType;
use inexor_rgf_graph::RelationTypeId;
use inexor_rgf_type_system_api::RelationTypeExportError;
use inexor_rgf_type_system_api::RelationTypeImportError;

#[async_trait]
pub trait RelationTypeImportExportManager: Send + Sync {
    /// Imports a relation type from a JSON file located at the given path.
    async fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError>;

    /// Exports the relation type with the given type id to a JSON file located at the given path.
    async fn export(&self, ty: &RelationTypeId, path: &str) -> Result<(), RelationTypeExportError>;
}
