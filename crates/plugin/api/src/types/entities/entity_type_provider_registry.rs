use std::sync::Arc;

use async_trait::async_trait;

use crate::TypeProvider;
use inexor_rgf_graph::EntityTypes;

#[async_trait]
pub trait EntityTypeProviderRegistry: Send + Sync {
    /// Registers an entity type provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<EntityTypes>>);

    /// Unregisters an entity type provider.
    async fn unregister_provider(&self, id: &str);
}
