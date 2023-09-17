use std::sync::Arc;

use async_trait::async_trait;

use crate::model::RelationTypes;
use crate::TypeProvider;

#[async_trait]
pub trait RelationTypeProviderRegistry: Send + Sync {
    /// Registers a relation type provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<RelationTypes>>);

    /// Unregisters a relation type provider.
    async fn unregister_provider(&self, id: &str);
}
