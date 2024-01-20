use std::sync::Arc;

use async_trait::async_trait;

use inexor_rgf_graph::RelationTypes;
use inexor_rgf_type_system_api::TypeProvider;

pub struct RelationTypeProviderRegistryDelegate(Arc<dyn inexor_rgf_type_system_api::RelationTypeProviderRegistry + Send + Sync>);

impl RelationTypeProviderRegistryDelegate {
    pub fn new(provider_registry: &Arc<dyn inexor_rgf_type_system_api::RelationTypeProviderRegistry + Send + Sync>) -> Self {
        Self(provider_registry.clone())
    }
}

#[async_trait]
impl inexor_rgf_plugin_api::RelationTypeProviderRegistry for RelationTypeProviderRegistryDelegate {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<RelationTypes>>) {
        self.0.register_provider(provider).await
    }

    async fn unregister_provider(&self, id: &str) {
        self.0.unregister_provider(id).await
    }
}
