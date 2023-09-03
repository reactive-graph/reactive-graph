use async_trait::async_trait;
use uuid::Uuid;

use crate::error::instances::entity::{EntityInstanceExportError, EntityInstanceImportError};
use crate::reactive::ReactiveEntity;

#[async_trait]
pub trait EntityInstanceImportExportManager: Send + Sync {
    /// Imports an entity instance from the given path.
    async fn import(&self, path: &str) -> Result<ReactiveEntity, EntityInstanceImportError>;

    /// Exports an entity instance to the given path.
    async fn export(&self, id: Uuid, path: &str) -> Result<(), EntityInstanceExportError>;
}
