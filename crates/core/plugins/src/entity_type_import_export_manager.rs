use crate::model::EntityType;
use crate::model::EntityTypeId;

#[derive(Debug)]
pub struct EntityTypeImportError;

#[derive(Debug)]
pub struct EntityTypeExportError;

pub trait EntityTypeImportExportManager: Send + Sync {
    /// Imports an entity type from a JSON file located at the given path.
    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError>;

    /// Exports the entity type with the given type id to a JSON file located at the given path.
    fn export(&self, ty: &EntityTypeId, path: &str) -> Result<(), EntityTypeExportError>;
}
