use async_trait::async_trait;
use std::sync::Arc;

use inexor_rgf_graph::Components;
use inexor_rgf_type_system_api::TypeProvider;

pub struct ComponentProviderRegistryDelegate(Arc<dyn inexor_rgf_type_system_api::ComponentProviderRegistry + Send + Sync>);

impl ComponentProviderRegistryDelegate {
    pub fn new(provider_registry: &Arc<dyn inexor_rgf_type_system_api::ComponentProviderRegistry + Send + Sync>) -> Self {
        Self(provider_registry.clone())
    }
}

#[async_trait]
impl inexor_rgf_plugin_api::ComponentProviderRegistry for ComponentProviderRegistryDelegate {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<Components>>) {
        self.0.register_provider(provider).await
    }

    async fn unregister_provider(&self, id: &str) {
        self.0.unregister_provider(id).await
    }
}
