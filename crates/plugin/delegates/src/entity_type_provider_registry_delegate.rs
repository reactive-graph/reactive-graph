use std::sync::Arc;

use async_trait::async_trait;

use inexor_rgf_graph::EntityTypes;
use inexor_rgf_type_system_api::TypeProvider;

pub struct EntityTypeProviderRegistryDelegate(Arc<dyn inexor_rgf_type_system_api::EntityTypeProviderRegistry + Send + Sync>);

impl EntityTypeProviderRegistryDelegate {
    pub fn new(provider_registry: &Arc<dyn inexor_rgf_type_system_api::EntityTypeProviderRegistry + Send + Sync>) -> Self {
        Self(provider_registry.clone())
    }
}

#[async_trait]
impl inexor_rgf_plugin_api::EntityTypeProviderRegistry for EntityTypeProviderRegistryDelegate {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<EntityTypes>>) {
        self.0.register_provider(provider).await
    }

    async fn unregister_provider(&self, id: &str) {
        self.0.unregister_provider(id).await
    }
}
