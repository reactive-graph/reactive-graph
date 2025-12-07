use reactive_graph_behaviour_model_api::prelude::*;

use reactive_graph_graph::RelationInstanceId;
use reactive_graph_reactive_model_api::ReactiveInstanceContainer;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_model_impl::ReactiveRelation;

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

    fn behaves_as_all(&self, tys: &BehaviourTypeIds) -> bool {
        self.reactive_instance.behaves_as_all(tys)
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
