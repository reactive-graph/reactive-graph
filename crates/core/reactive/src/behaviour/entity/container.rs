use std::sync::Arc;

use crate::model::BehaviourTypeId;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveEntityInstance;
use crate::BehaviourReactiveInstanceContainer;

pub struct EntityBehaviourReactiveInstanceContainerImpl {
    pub reactive_instance: Arc<ReactiveEntityInstance>,
}

impl BehaviourReactiveInstanceContainer<ReactiveEntityInstance> for EntityBehaviourReactiveInstanceContainerImpl {
    fn get_reactive_instance(&self) -> &Arc<ReactiveEntityInstance> {
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
