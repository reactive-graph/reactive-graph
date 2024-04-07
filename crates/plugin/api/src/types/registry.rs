use crate::TypeProvider;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait TypesProviderRegistry<T: reactive_graph_graph::NamespacedTypeContainer>: Send + Sync {
    /// Registers a component provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<T>>);

    /// Unregisters a component provider.
    async fn unregister_provider(&self, id: &str);
}
