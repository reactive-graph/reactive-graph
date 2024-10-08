use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_stream::StreamExt;

use reactive_graph_behaviour_model_api::prelude::*;

use reactive_graph_graph::RelationInstanceId;
use reactive_graph_reactive_model_impl::ReactiveRelation;

#[async_trait]
pub trait RelationBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating relation behaviours.
    /// If a relation instance is of the relation type then the given behaviour is applied.
    /// The behaviour will be created using the given RelationBehaviourCreator.
    #[allow(unused_variables)]
    async fn register(
        &self,
        relation_behaviour_ty: RelationBehaviourTypeId,
        factory: Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>,
    );

    async fn register_all(&self, factories: &BehaviourFactories<RelationInstanceId, ReactiveRelation>) {
        let mut factories = tokio_stream::iter(factories.deref().clone());
        while let Some((ty, factory)) = factories.next().await {
            self.register(RelationBehaviourTypeId::from(&ty), factory).await
        }
    }

    /// Unregisters a factory for creating relation behaviours.
    #[allow(unused_variables)]
    async fn unregister(&self, relation_behaviour_ty: &RelationBehaviourTypeId);

    /// Unregisters the behaviour factories for the given relation behaviour types.
    async fn unregister_all(&self, tys: &RelationBehaviourTypeIds) {
        let mut tys = tokio_stream::iter(tys.iter());
        while let Some(ty) = tys.next().await {
            self.unregister(&ty).await
        }
    }
}
