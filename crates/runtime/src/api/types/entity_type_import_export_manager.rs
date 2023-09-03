use async_trait::async_trait;

use crate::error::types::entity::EntityTypeExportError;
use crate::error::types::entity::EntityTypeImportError;
use crate::model::EntityType;
use crate::model::EntityTypeId;

#[async_trait]
pub trait EntityTypeImportExportManager: Send + Sync {
    /// Imports an entity type from a JSON file file located at the given path.
    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError>;

    /// Exports the entity type with the given name to a JSON file located at the given path.
    fn export(&self, ty: &EntityTypeId, path: &str) -> Result<(), EntityTypeExportError>;
}
