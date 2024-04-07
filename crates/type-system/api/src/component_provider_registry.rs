use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use crate::TypeProvider;
use reactive_graph_graph::Components;
use reactive_graph_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait ComponentProviderRegistry: Send + Sync + Lifecycle {
    /// Registers a component provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<Components>>);

    /// Unregisters a component provider.
    async fn unregister_provider(&self, id: &str);
}
