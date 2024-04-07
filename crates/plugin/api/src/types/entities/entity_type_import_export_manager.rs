use async_trait::async_trait;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_type_system_api::EntityTypeExportError;
use reactive_graph_type_system_api::EntityTypeImportError;

#[async_trait]
pub trait EntityTypeImportExportManager: Send + Sync {
    /// Imports an entity type from a JSON file located at the given path.
    async fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError>;

    /// Exports the entity type with the given type id to a JSON file located at the given path.
    async fn export(&self, ty: &EntityTypeId, path: &str) -> Result<(), EntityTypeExportError>;
}
