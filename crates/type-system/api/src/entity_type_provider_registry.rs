use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use crate::TypeProvider;
use reactive_graph_graph::EntityTypes;
use reactive_graph_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait EntityTypeProviderRegistry: Send + Sync + Lifecycle {
    /// Registers an entity type provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<EntityTypes>>);

    /// Unregisters an entity type provider.
    async fn unregister_provider(&self, id: &str);
}
