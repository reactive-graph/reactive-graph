use async_trait::async_trait;

use crate::model::RelationType;
use crate::model::RelationTypeId;
use crate::rt_api::RelationTypeExportError;
use crate::rt_api::RelationTypeImportError;

#[async_trait]
pub trait RelationTypeImportExportManager: Send + Sync {
    /// Imports a relation type from a JSON file file located at the given path.
    fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError>;

    /// Exports the relation type with the given name to a JSON file located at the given path.
    fn export(&self, ty: &RelationTypeId, path: &str) -> Result<(), RelationTypeExportError>;
}
