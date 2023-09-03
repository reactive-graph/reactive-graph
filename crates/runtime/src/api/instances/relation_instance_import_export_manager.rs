use async_trait::async_trait;

use crate::error::instances::relation::{RelationInstanceExportError, RelationInstanceImportError};
use crate::reactive::ReactiveRelation;
use crate::model::RelationInstanceId;

#[async_trait]
pub trait RelationInstanceImportExportManager: Send + Sync {
    async fn import(&self, path: &str) -> Result<ReactiveRelation, RelationInstanceImportError>;

    async fn export(&self, id: &RelationInstanceId, path: &str) -> Result<(), RelationInstanceExportError>;
}
