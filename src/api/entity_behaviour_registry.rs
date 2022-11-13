use std::sync::Arc;

use async_trait::async_trait;

use crate::model::EntityBehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model::ReactiveEntityInstance;
use crate::reactive::BehaviourFactory;

#[async_trait]
pub trait EntityBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating entity behaviours.
    fn register(&self, entity_behaviour_ty: EntityBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>);

    /// Unregisters a factory for creating entity behaviours.
    fn unregister(&self, entity_behaviour_ty: &EntityBehaviourTypeId);

    /// Returns the entity behaviour factories for the given entity type.
    fn get(&self, entity_ty: &EntityTypeId) -> Vec<Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>>;
}
