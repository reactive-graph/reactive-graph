use async_trait::async_trait;

use crate::model::RelationInstanceId;
use crate::reactive::ReactiveRelation;
use crate::rt_api::RelationInstanceExportError;
use crate::rt_api::RelationInstanceImportError;

#[async_trait]
pub trait RelationInstanceImportExportManager: Send + Sync {
    async fn import(&self, path: &str) -> Result<ReactiveRelation, RelationInstanceImportError>;

    async fn export(&self, id: &RelationInstanceId, path: &str) -> Result<(), RelationInstanceExportError>;
}
