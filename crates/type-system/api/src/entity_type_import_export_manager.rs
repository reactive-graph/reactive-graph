use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_graph::EntityType;
use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_lifecycle::Lifecycle;

use crate::EntityTypeExportError;
use crate::EntityTypeImportError;

#[injectable]
#[async_trait]
pub trait EntityTypeImportExportManager: Send + Sync + Lifecycle {
    /// Imports an entity type from a JSON file file located at the given path.
    async fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError>;

    /// Exports the entity type with the given name to a JSON file located at the given path.
    async fn export(&self, ty: &EntityTypeId, path: &str) -> Result<(), EntityTypeExportError>;
}
