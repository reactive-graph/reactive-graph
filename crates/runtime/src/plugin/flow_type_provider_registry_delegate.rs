use std::sync::Arc;

use async_trait::async_trait;

use crate::model::FlowTypes;
use crate::plugins::FlowTypeProviderRegistry;
use crate::plugins::TypeProvider;

pub struct FlowTypeProviderRegistryDelegate(Arc<dyn crate::api::FlowTypeProviderRegistry>);

impl FlowTypeProviderRegistryDelegate {
    pub fn new(provider_registry: &Arc<dyn crate::api::FlowTypeProviderRegistry>) -> Self {
        Self(provider_registry.clone())
    }
}

#[async_trait]
impl FlowTypeProviderRegistry for FlowTypeProviderRegistryDelegate {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<FlowTypes>>) {
        self.0.register_provider(provider).await
    }

    async fn unregister_provider(&self, id: &str) {
        self.0.unregister_provider(id).await
    }
}
