use std::sync::Arc;

use async_trait::async_trait;

use crate::model::EntityTypes;
use crate::plugins::EntityTypeProviderRegistry;
use crate::plugins::TypeProvider;

pub struct EntityTypeProviderRegistryDelegate(Arc<dyn crate::api::EntityTypeProviderRegistry>);

impl EntityTypeProviderRegistryDelegate {
    pub fn new(provider_registry: &Arc<dyn crate::api::EntityTypeProviderRegistry>) -> Self {
        Self(provider_registry.clone())
    }
}

#[async_trait]
impl EntityTypeProviderRegistry for EntityTypeProviderRegistryDelegate {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<EntityTypes>>) {
        self.0.register_provider(provider).await
    }

    async fn unregister_provider(&self, id: &str) {
        self.0.unregister_provider(id).await
    }
}
