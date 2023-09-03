use async_trait::async_trait;
use uuid::Uuid;

use crate::behaviour::BehaviourTransitionError;
use crate::reactive::BehaviourTypeId;
use crate::reactive::ReactiveEntity;

#[async_trait]
pub trait EntityBehaviourManager: Send + Sync {
    /// Adds all behaviours to the given reactive entity instance.
    fn add_behaviours(&self, entity_instance: ReactiveEntity);

    /// Creates and adds the given behaviour to the given reactive entity instance.
    fn add_behaviour(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId);

    /// Removes the given behaviour from the given reactive entity instance.
    fn remove_behaviour(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId);

    /// Removes all behaviours from the given reactive entity instance.
    fn remove_behaviours(&self, entity_instance: ReactiveEntity);

    /// Removes all behaviours from the reactive entity instance with the given id.
    fn remove_behaviours_by_id(&self, id: &Uuid);

    /// Removes all behaviours of the given behaviour type.
    fn remove_behaviours_by_behaviour(&self, behaviour_ty: &BehaviourTypeId);

    /// Returns true, if the entity instance has the given behaviour.
    fn has(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) -> bool;

    /// Returns the behaviours of the given entity instance.
    fn get_all(&self, entity_instance: ReactiveEntity) -> Vec<BehaviourTypeId>;

    /// Returns the entity instances with the given behaviour.
    fn get_instances_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<ReactiveEntity>;

    /// Connect
    fn connect(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Disconnect
    fn disconnect(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Reconnect
    fn reconnect(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;
}
