use async_trait::async_trait;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::TypeSystemProvider;

#[async_trait]
pub trait TypeSystemProviderRegistry: Send + Sync {
    /// Registers a component provider.
    async fn register_provider(&self, provider: TypeSystemProvider);

    /// Unregisters a component provider.
    async fn unregister_provider(&self, id: &Namespace);
}
