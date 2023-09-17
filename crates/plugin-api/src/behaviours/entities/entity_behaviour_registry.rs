use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_stream::StreamExt;
use uuid::Uuid;

use inexor_rgf_behaviour_api::prelude::*;

use crate::reactive::ReactiveEntity;

#[async_trait]
pub trait EntityBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating entity behaviours.
    /// If an entity instance is of the entity type then the given behaviour is applied.
    /// The behaviour will be created using the given EntityBehaviourCreator.
    #[allow(unused_variables)]
    async fn register(&self, entity_behaviour_ty: EntityBehaviourTypeId, factory: Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>);

    async fn register_all(&self, factories: &BehaviourFactories<Uuid, ReactiveEntity>) {
        let mut factories = tokio_stream::iter(factories.deref().clone().into_iter());
        while let Some((ty, factory)) = factories.next().await {
            self.register(EntityBehaviourTypeId::from(&ty), factory).await
        }
    }

    /// Unregisters a factory for creating entity behaviours.
    #[allow(unused_variables)]
    async fn unregister(&self, entity_behaviour_ty: &EntityBehaviourTypeId);

    /// Unregisters the behaviour factories for the given entity behaviour types.
    async fn unregister_all(&self, tys: &EntityBehaviourTypeIds) {
        let mut tys = tokio_stream::iter(tys.iter());
        while let Some(ty) = tys.next().await {
            self.unregister(&ty).await
        }
    }
}
