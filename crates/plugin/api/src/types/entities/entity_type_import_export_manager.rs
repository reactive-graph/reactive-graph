use async_trait::async_trait;
use inexor_rgf_graph::EntityType;
use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_type_system_api::EntityTypeExportError;
use inexor_rgf_type_system_api::EntityTypeImportError;

#[async_trait]
pub trait EntityTypeImportExportManager: Send + Sync {
    /// Imports an entity type from a JSON file located at the given path.
    async fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError>;

    /// Exports the entity type with the given type id to a JSON file located at the given path.
    async fn export(&self, ty: &EntityTypeId, path: &str) -> Result<(), EntityTypeExportError>;
}
