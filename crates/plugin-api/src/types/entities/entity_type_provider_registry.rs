use std::sync::Arc;

use async_trait::async_trait;

use crate::model::EntityTypes;
use crate::TypeProvider;

#[async_trait]
pub trait EntityTypeProviderRegistry: Send + Sync {
    /// Registers an entity type provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<EntityTypes>>);

    /// Unregisters an entity type provider.
    async fn unregister_provider(&self, id: &str);
}
