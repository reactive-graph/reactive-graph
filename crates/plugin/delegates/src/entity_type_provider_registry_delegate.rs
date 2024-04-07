use std::sync::Arc;

use async_trait::async_trait;

use reactive_graph_graph::EntityTypes;
use reactive_graph_type_system_api::TypeProvider;

pub struct EntityTypeProviderRegistryDelegate(Arc<dyn reactive_graph_type_system_api::EntityTypeProviderRegistry + Send + Sync>);

impl EntityTypeProviderRegistryDelegate {
    pub fn new(provider_registry: &Arc<dyn reactive_graph_type_system_api::EntityTypeProviderRegistry + Send + Sync>) -> Self {
        Self(provider_registry.clone())
    }
}

#[async_trait]
impl reactive_graph_plugin_api::EntityTypeProviderRegistry for EntityTypeProviderRegistryDelegate {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<EntityTypes>>) {
        self.0.register_provider(provider).await
    }

    async fn unregister_provider(&self, id: &str) {
        self.0.unregister_provider(id).await
    }
}
