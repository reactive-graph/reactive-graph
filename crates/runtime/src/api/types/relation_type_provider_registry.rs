use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::RelationTypes;
use crate::plugins::TypeProvider;

#[async_trait]
pub trait RelationTypeProviderRegistry: Send + Sync + Lifecycle {
    /// Registers a relation type provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<RelationTypes>>);

    /// Unregisters a relation type provider.
    async fn unregister_provider(&self, id: &str);
}
