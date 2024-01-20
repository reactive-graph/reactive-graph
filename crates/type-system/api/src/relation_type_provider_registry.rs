use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use crate::TypeProvider;
use inexor_rgf_graph::RelationTypes;
use inexor_rgf_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait RelationTypeProviderRegistry: Send + Sync + Lifecycle {
    /// Registers a relation type provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<RelationTypes>>);

    /// Unregisters a relation type provider.
    async fn unregister_provider(&self, id: &str);
}
