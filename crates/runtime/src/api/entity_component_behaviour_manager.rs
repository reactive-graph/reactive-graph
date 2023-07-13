use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::model::BehaviourTypeId;
use crate::model::Component;
use crate::model::ComponentBehaviourTypeId;
use crate::model::ReactiveEntityInstance;
use crate::reactive::BehaviourTransitionError;

#[async_trait]
pub trait EntityComponentBehaviourManager: Send + Sync {
    /// Adds new behaviours to the given entity instance.
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Possibly adds new behaviour to the given entity instance's component
    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: Component);

    /// Creates and adds the given behaviour to the given reactive entity instance's component.
    fn add_behaviour_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component_behaviour_ty: &ComponentBehaviourTypeId);

    /// Removes the given behaviour from the given reactive entity instance.
    fn remove_behaviour_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId);

    /// Removes behaviours from the given entity instance.
    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Removes behaviour from the given entity instance's component
    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: Component);

    /// Removes behaviours from the given entity instance by uuid.
    fn remove_behaviours_by_id(&self, id: &Uuid);

    /// Removes all behaviours of the given behaviour type.
    fn remove_behaviours_by_behaviour(&self, behaviour_ty: &BehaviourTypeId);

    /// Returns true, if the entity instance has the given behaviour.
    fn has(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> bool;

    /// Returns the behaviours of the given entity instance.
    fn get_all(&self, entity_instance: Arc<ReactiveEntityInstance>) -> Vec<BehaviourTypeId>;

    /// Returns the entity instances with the given behaviour.
    fn get_instances_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<Arc<ReactiveEntityInstance>>;

    /// Connect
    fn connect(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Disconnect
    fn disconnect(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Reconnect
    fn reconnect(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;
}