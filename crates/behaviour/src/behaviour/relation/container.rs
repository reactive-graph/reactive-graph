use inexor_rgf_core_model::RelationInstanceId;
use crate::reactive::BehaviourTypeId;
use crate::reactive::ReactiveBehaviourContainer;
use crate::reactive::ReactiveEntity;
use crate::reactive::ReactiveRelation;
use crate::BehaviourReactiveInstanceContainer;

pub trait BehaviourRelationInstanceContainer: BehaviourReactiveInstanceContainer<RelationInstanceId, ReactiveRelation> {
    /// Returns the outbound instance of the behaviour.
    fn get_outbound(&self) -> &ReactiveEntity;

    /// Returns the inbound instance of the behaviour.
    fn get_inbound(&self) -> &ReactiveEntity;
}

pub struct RelationBehaviourReactiveInstanceContainerImpl {
    pub reactive_instance: ReactiveRelation,
}

impl BehaviourReactiveInstanceContainer<RelationInstanceId, ReactiveRelation> for RelationBehaviourReactiveInstanceContainerImpl {
    fn get_reactive_instance(&self) -> &ReactiveRelation {
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
    fn get_outbound(&self) -> &ReactiveEntity {
        &self.reactive_instance.outbound
    }

    fn get_inbound(&self) -> &ReactiveEntity {
        &self.reactive_instance.inbound
    }
}
