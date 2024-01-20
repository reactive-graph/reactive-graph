use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_stream::StreamExt;

use inexor_rgf_behaviour_model_api::prelude::*;

use inexor_rgf_graph::RelationInstanceId;
use inexor_rgf_reactive_model_impl::ReactiveRelation;

#[async_trait]
pub trait RelationComponentBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating relation component behaviours.
    /// If a relation instance has the given component then the given behaviour is applied.
    /// The behaviour will be created using the given RelationBehaviourCreator.
    #[allow(unused_variables)]
    async fn register(
        &self,
        component_behaviour_ty: ComponentBehaviourTypeId,
        factory: Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>,
    );

    async fn register_all(&self, factories: &BehaviourFactories<RelationInstanceId, ReactiveRelation>) {
        let mut factories = tokio_stream::iter(factories.deref().clone().into_iter());
        while let Some((ty, factory)) = factories.next().await {
            self.register(ComponentBehaviourTypeId::from(&ty), factory).await
        }
    }

    /// Unregisters a factory for creating relation component behaviours.
    #[allow(unused_variables)]
    async fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId);

    /// Unregisters the behaviour factories for the given component behaviour types.
    async fn unregister_all(&self, tys: &ComponentBehaviourTypeIds) {
        let mut tys = tokio_stream::iter(tys.iter());
        while let Some(ty) = tys.next().await {
            self.unregister(&ty).await
        }
    }
}
