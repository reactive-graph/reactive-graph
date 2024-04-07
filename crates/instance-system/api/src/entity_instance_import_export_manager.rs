use async_trait::async_trait;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;
use uuid::Uuid;

use reactive_graph_reactive_model_impl::ReactiveEntity;

use crate::EntityInstanceExportError;
use crate::EntityInstanceImportError;

#[injectable]
#[async_trait]
pub trait EntityInstanceImportExportManager: Send + Sync + Lifecycle {
    /// Imports an entity instance from the given path.
    async fn import(&self, path: &str) -> Result<ReactiveEntity, EntityInstanceImportError>;

    /// Exports an entity instance to the given path.
    async fn export(&self, id: Uuid, path: &str) -> Result<(), EntityInstanceExportError>;
}
