use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_graph::RelationInstanceId;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveRelation;

use crate::RelationInstanceExportError;
use crate::RelationInstanceImportError;

#[injectable]
#[async_trait]
pub trait RelationInstanceImportExportManager: Send + Sync + Lifecycle {
    async fn import(&self, path: &str) -> Result<ReactiveRelation, RelationInstanceImportError>;

    async fn export(&self, id: &RelationInstanceId, path: &str) -> Result<(), RelationInstanceExportError>;
}
