use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_graph::RelationType;
use inexor_rgf_graph::RelationTypeId;
use inexor_rgf_lifecycle::Lifecycle;

use crate::RelationTypeExportError;
use crate::RelationTypeImportError;

#[injectable]
#[async_trait]
pub trait RelationTypeImportExportManager: Send + Sync + Lifecycle {
    /// Imports a relation type from a JSON file file located at the given path.
    async fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError>;

    /// Exports the relation type with the given name to a JSON file located at the given path.
    async fn export(&self, ty: &RelationTypeId, path: &str) -> Result<(), RelationTypeExportError>;
}
