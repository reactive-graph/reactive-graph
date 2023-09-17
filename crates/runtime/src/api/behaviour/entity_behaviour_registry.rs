use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use inexor_rgf_behaviour_api::prelude::*;

use crate::model::EntityTypeId;
use crate::reactive::ReactiveEntity;

#[async_trait]
pub trait EntityBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating entity behaviours.
    fn register(&self, entity_behaviour_ty: EntityBehaviourTypeId, factory: Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>);

    /// Unregisters a factory for creating entity behaviours.
    fn unregister(&self, entity_behaviour_ty: &EntityBehaviourTypeId);

    /// Returns all entity behaviours.
    fn get_all(&self) -> Vec<EntityBehaviourTypeId>;

    /// Returns the entity behaviour factories for the given entity type.
    fn get(&self, entity_ty: &EntityTypeId) -> Vec<Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>>;

    /// Returns the entity behaviour for the given behaviour type if the entity behaviour exists.
    fn get_factory_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>>;

    /// Returns the entity behaviours for the given entity type.
    fn get_behaviour_types(&self, entity_ty: &EntityTypeId) -> Vec<EntityBehaviourTypeId>;

    /// Returns the entity behaviour for the given behaviour type if the entity behaviour exists.
    fn get_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<EntityBehaviourTypeId>;

    /// Returns the count of entity behaviours.
    fn count(&self) -> usize;
}
