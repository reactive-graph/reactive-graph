use std::sync::Arc;

use async_trait::async_trait;

use crate::TypeProvider;
use reactive_graph_graph::Components;

#[async_trait]
pub trait ComponentProviderRegistry: Send + Sync {
    /// Registers a component provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<Components>>);

    /// Unregisters a component provider.
    async fn unregister_provider(&self, id: &str);
}
