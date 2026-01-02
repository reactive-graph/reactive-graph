use async_trait::async_trait;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::TypeSystemProvider;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait TypeSystemProviderRegistry: Send + Sync + Lifecycle {
    /// Registers a component provider.
    async fn register_provider(&self, provider: TypeSystemProvider);

    /// Unregisters a component provider.
    async fn unregister_provider(&self, id: &Namespace);
}
