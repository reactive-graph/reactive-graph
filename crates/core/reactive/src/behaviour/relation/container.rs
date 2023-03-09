use std::sync::Arc;

use crate::model::BehaviourTypeId;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveRelationInstance;
use crate::BehaviourReactiveInstanceContainer;

pub trait BehaviourRelationInstanceContainer: BehaviourReactiveInstanceContainer<ReactiveRelationInstance> {
    /// Returns the outbound instance of the behaviour.
    fn get_outbound(&self) -> &Arc<ReactiveEntityInstance>;

    /// Returns the inbound instance of the behaviour.
    fn get_inbound(&self) -> &Arc<ReactiveEntityInstance>;
}

pub struct RelationBehaviourReactiveInstanceContainerImpl {
    pub reactive_instance: Arc<ReactiveRelationInstance>,
}

impl BehaviourReactiveInstanceContainer<ReactiveRelationInstance> for RelationBehaviourReactiveInstanceContainerImpl {
    fn get_reactive_instance(&self) -> &Arc<ReactiveRelationInstance> {
        &self.reactive_instance
    }
}

impl ReactiveBehaviourContainer for RelationBehaviourReactiveInstanceContainerImpl {
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

impl BehaviourRelationInstanceContainer for RelationBehaviourReactiveInstanceContainerImpl {
    fn get_outbound(&self) -> &Arc<ReactiveEntityInstance> {
        &self.reactive_instance.outbound
    }

    fn get_inbound(&self) -> &Arc<ReactiveEntityInstance> {
        &self.reactive_instance.inbound
    }
}
