use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_reactive_api::prelude::*;

use crate::model::RelationInstanceId;
use crate::reactive::BehaviourTypesContainer;
use crate::reactive::ReactiveEntity;
use crate::reactive::ReactiveRelation;

pub trait BehaviourRelationInstanceContainer: ReactiveInstanceContainer<RelationInstanceId, ReactiveRelation> {
    /// Returns the outbound instance of the behaviour.
    fn get_outbound(&self) -> &ReactiveEntity;

    /// Returns the inbound instance of the behaviour.
    fn get_inbound(&self) -> &ReactiveEntity;
}

pub struct RelationReactiveInstanceContainerImpl {
    pub reactive_instance: ReactiveRelation,
}

impl ReactiveInstanceContainer<RelationInstanceId, ReactiveRelation> for RelationReactiveInstanceContainerImpl {
    fn get_reactive_instance(&self) -> &ReactiveRelation {
        &self.reactive_instance
    }
}

impl BehaviourTypesContainer for RelationReactiveInstanceContainerImpl {
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

impl BehaviourRelationInstanceContainer for RelationReactiveInstanceContainerImpl {
    fn get_outbound(&self) -> &ReactiveEntity {
        &self.reactive_instance.outbound
    }

    fn get_inbound(&self) -> &ReactiveEntity {
        &self.reactive_instance.inbound
    }
}
