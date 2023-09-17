use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::rt_api::EntityTypeExportError;
use crate::rt_api::EntityTypeImportError;
use async_trait::async_trait;

#[async_trait]
pub trait EntityTypeImportExportManager: Send + Sync {
    /// Imports an entity type from a JSON file located at the given path.
    async fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError>;

    /// Exports the entity type with the given type id to a JSON file located at the given path.
    async fn export(&self, ty: &EntityTypeId, path: &str) -> Result<(), EntityTypeExportError>;
}
