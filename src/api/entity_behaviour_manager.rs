use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::model::BehaviourTypeId;
use crate::model::ReactiveEntityInstance;
use crate::reactive::BehaviourTransitionError;

#[async_trait]
pub trait EntityBehaviourManager: Send + Sync {
    /// Adds all behaviours to the given reactive entity instance.
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Removes all behaviours from the given reactive entity instance.
    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Removes all behaviours from the reactive entity instance with the given id.
    fn remove_behaviours_by_id(&self, id: &Uuid);

    /// Connect
    fn connect(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Disconnect
    fn disconnect(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Reconnect
    fn reconnect(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;
}
