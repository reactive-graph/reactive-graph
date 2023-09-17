use std::sync::Arc;

use async_trait::async_trait;

use crate::model::RelationTypes;
use crate::plugins::RelationTypeProviderRegistry;
use crate::plugins::TypeProvider;

pub struct RelationTypeProviderRegistryDelegate(Arc<dyn crate::api::RelationTypeProviderRegistry>);

impl RelationTypeProviderRegistryDelegate {
    pub fn new(provider_registry: &Arc<dyn crate::api::RelationTypeProviderRegistry>) -> Self {
        Self(provider_registry.clone())
    }
}

#[async_trait]
impl RelationTypeProviderRegistry for RelationTypeProviderRegistryDelegate {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<RelationTypes>>) {
        self.0.register_provider(provider).await
    }

    async fn unregister_provider(&self, id: &str) {
        self.0.unregister_provider(id).await
    }
}
