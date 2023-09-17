use async_trait::async_trait;

use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::rt_api::EntityTypeExportError;
use crate::rt_api::EntityTypeImportError;

#[async_trait]
pub trait EntityTypeImportExportManager: Send + Sync {
    /// Imports an entity type from a JSON file file located at the given path.
    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError>;

    /// Exports the entity type with the given name to a JSON file located at the given path.
    fn export(&self, ty: &EntityTypeId, path: &str) -> Result<(), EntityTypeExportError>;
}
