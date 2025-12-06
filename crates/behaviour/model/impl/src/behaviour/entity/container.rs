use uuid::Uuid;

use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_reactive_model_api::ReactiveInstanceContainer;
use reactive_graph_reactive_model_impl::ReactiveEntity;

pub struct EntityReactiveInstanceContainerImpl {
    pub reactive_instance: ReactiveEntity,
}

impl ReactiveInstanceContainer<Uuid, ReactiveEntity> for EntityReactiveInstanceContainerImpl {
    fn get_reactive_instance(&self) -> &ReactiveEntity {
        &self.reactive_instance
    }
}

impl BehaviourTypesContainer for EntityReactiveInstanceContainerImpl {
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
