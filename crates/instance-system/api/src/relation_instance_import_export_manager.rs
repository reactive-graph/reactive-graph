use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_graph::RelationInstanceId;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_model_impl::ReactiveRelation;

use crate::RelationInstanceExportError;
use crate::RelationInstanceImportError;

#[injectable]
#[async_trait]
pub trait RelationInstanceImportExportManager: Send + Sync + Lifecycle {
    async fn import(&self, path: &str) -> Result<ReactiveRelation, RelationInstanceImportError>;

    async fn export(&self, id: &RelationInstanceId, path: &str) -> Result<(), RelationInstanceExportError>;
}
