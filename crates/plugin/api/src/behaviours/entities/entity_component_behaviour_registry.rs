use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_stream::StreamExt;
use uuid::Uuid;

use inexor_rgf_behaviour_model_api::prelude::*;

use inexor_rgf_reactive_model_impl::ReactiveEntity;

#[async_trait]
pub trait EntityComponentBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating entity component behaviours.
    /// If an entity instance has the given component then the given behaviour is applied.
    /// The behaviour will be created using the given EntityBehaviourCreator.
    #[allow(unused_variables)]
    async fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>);

    async fn register_all(&self, factories: &BehaviourFactories<Uuid, ReactiveEntity>) {
        let mut factories = tokio_stream::iter(factories.deref().clone().into_iter());
        while let Some((ty, factory)) = factories.next().await {
            self.register(ComponentBehaviourTypeId::from(&ty), factory).await
        }
    }

    /// Unregisters an entity component behaviour factory.
    #[allow(unused_variables)]
    async fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId);

    /// Unregisters all factories with the given entity component behaviour types.
    async fn unregister_all(&self, tys: &ComponentBehaviourTypeIds) {
        let mut tys = tokio_stream::iter(tys.iter());
        while let Some(ty) = tys.next().await {
            self.unregister(&ty).await
        }
    }
}
