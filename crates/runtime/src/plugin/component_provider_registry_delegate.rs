use async_trait::async_trait;
use std::sync::Arc;

use crate::model::Components;
use crate::plugins::ComponentProviderRegistry;
use crate::plugins::TypeProvider;

pub struct ComponentProviderRegistryDelegate(Arc<dyn crate::api::ComponentProviderRegistry>);

impl ComponentProviderRegistryDelegate {
    pub fn new(provider_registry: &Arc<dyn crate::api::ComponentProviderRegistry>) -> Self {
        Self(provider_registry.clone())
    }
}

#[async_trait]
impl ComponentProviderRegistry for ComponentProviderRegistryDelegate {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<Components>>) {
        self.0.register_provider(provider).await
    }

    async fn unregister_provider(&self, id: &str) {
        self.0.unregister_provider(id).await
    }
}
