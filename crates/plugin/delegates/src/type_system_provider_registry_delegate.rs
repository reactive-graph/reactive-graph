use async_trait::async_trait;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::TypeSystemProvider;
use std::sync::Arc;

pub struct TypeSystemProviderRegistryDelegate(Arc<dyn reactive_graph_type_system_api::TypeSystemProviderRegistry + Send + Sync>);

impl TypeSystemProviderRegistryDelegate {
    pub fn new(provider_registry: &Arc<dyn reactive_graph_type_system_api::TypeSystemProviderRegistry + Send + Sync>) -> Self {
        Self(provider_registry.clone())
    }
}

#[async_trait]
impl reactive_graph_plugin_api::TypeSystemProviderRegistry for TypeSystemProviderRegistryDelegate {
    async fn register_provider(&self, provider: TypeSystemProvider) {
        self.0.register_provider(provider).await
    }

    async fn unregister_provider(&self, id: &Namespace) {
        self.0.unregister_provider(id).await
    }
}
