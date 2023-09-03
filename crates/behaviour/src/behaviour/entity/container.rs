use uuid::Uuid;

use crate::BehaviourReactiveInstanceContainer;
use crate::reactive::BehaviourTypeId;
use crate::reactive::ReactiveBehaviourContainer;
use crate::reactive::ReactiveEntity;

pub struct EntityBehaviourReactiveInstanceContainerImpl {
    pub reactive_instance: ReactiveEntity,
}

impl BehaviourReactiveInstanceContainer<Uuid, ReactiveEntity> for EntityBehaviourReactiveInstanceContainerImpl {
    fn get_reactive_instance(&self) -> &ReactiveEntity {
        &self.reactive_instance
    }
}

impl ReactiveBehaviourContainer for EntityBehaviourReactiveInstanceContainerImpl {
    fn get_behaviours(&self) -> Vec<BehaviourTypeId> {
        self.reactive_instance.get_behaviours()
    }

    fn add_behaviour(&self, ty: BehaviourTypeId) {
        self.reactive_instance.add_behaviour(ty);
    }

    fn remove_behaviour(&self, ty: &BehaviourTypeId) {
        self.reactive_instance.remove_behaviour(ty);
    }

    fn behaves_as(&self, ty: &BehaviourTypeId) -> bool {
        self.reactive_instance.behaves_as(ty)
    }
}
