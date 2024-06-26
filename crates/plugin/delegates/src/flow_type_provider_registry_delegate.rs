use std::sync::Arc;

use async_trait::async_trait;

use reactive_graph_graph::FlowTypes;
use reactive_graph_type_system_api::TypeProvider;

pub struct FlowTypeProviderRegistryDelegate(Arc<dyn reactive_graph_type_system_api::FlowTypeProviderRegistry + Send + Sync>);

impl FlowTypeProviderRegistryDelegate {
    pub fn new(provider_registry: &Arc<dyn reactive_graph_type_system_api::FlowTypeProviderRegistry + Send + Sync>) -> Self {
        Self(provider_registry.clone())
    }
}

#[async_trait]
impl reactive_graph_plugin_api::FlowTypeProviderRegistry for FlowTypeProviderRegistryDelegate {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<FlowTypes>>) {
        self.0.register_provider(provider).await
    }

    async fn unregister_provider(&self, id: &str) {
        self.0.unregister_provider(id).await
    }
}
