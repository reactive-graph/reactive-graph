use std::sync::Arc;

use async_trait::async_trait;

use reactive_graph_graph::RelationTypes;
use reactive_graph_type_system_api::TypeProvider;

pub struct RelationTypeProviderRegistryDelegate(Arc<dyn reactive_graph_type_system_api::RelationTypeProviderRegistry + Send + Sync>);

impl RelationTypeProviderRegistryDelegate {
    pub fn new(provider_registry: &Arc<dyn reactive_graph_type_system_api::RelationTypeProviderRegistry + Send + Sync>) -> Self {
        Self(provider_registry.clone())
    }
}

#[async_trait]
impl reactive_graph_plugin_api::RelationTypeProviderRegistry for RelationTypeProviderRegistryDelegate {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<RelationTypes>>) {
        self.0.register_provider(provider).await
    }

    async fn unregister_provider(&self, id: &str) {
        self.0.unregister_provider(id).await
    }
}
