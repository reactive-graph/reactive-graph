use async_trait::async_trait;
use log::trace;
use std::sync::Arc;

use reactive_graph_behaviour_model_api::BehaviourConnectFailed;
use reactive_graph_behaviour_model_api::BehaviourDisconnectFailed;
use reactive_graph_behaviour_model_api::BehaviourState;
use reactive_graph_behaviour_model_api::BehaviourTransitionError;
use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_behaviour_model_impl::RelationBehaviourStorage;
use reactive_graph_behaviour_service_api::RelationBehaviourManager;
use reactive_graph_behaviour_service_api::RelationBehaviourRegistry;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_api::ReactiveInstance;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use springtime_di::Component;
use springtime_di::component_alias;

#[derive(Component)]
pub struct RelationBehaviourManagerImpl {
    relation_behaviour_registry: Arc<dyn RelationBehaviourRegistry + Send + Sync>,

    #[component(default = "RelationBehaviourStorage::new")]
    relation_behaviour_storage: RelationBehaviourStorage,
}

#[async_trait]
#[component_alias]
impl RelationBehaviourManager for RelationBehaviourManagerImpl {
    fn add_behaviours(&self, relation_instance: ReactiveRelation) {
        let id = relation_instance.id();
        let relation_ty = relation_instance.relation_type_id();
        for factory in self.relation_behaviour_registry.get(&relation_ty) {
            if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                self.relation_behaviour_storage.insert(id.clone(), behaviour.ty().clone(), behaviour.clone());
                trace!("Added relation behaviour {}", behaviour.ty());
            }
        }
    }

    fn add_behaviour(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) {
        if let Some(factory) = self.relation_behaviour_registry.get_factory_by_behaviour_type(behaviour_ty) {
            let id = relation_instance.id();
            if let Ok(behaviour) = factory.create(relation_instance) {
                let behaviour_ty = behaviour.ty().clone();
                self.relation_behaviour_storage.insert(id, behaviour_ty.clone(), behaviour);
                trace!("Added relation behaviour {}", &behaviour_ty);
            }
        }
    }

    fn remove_behaviour(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) {
        let id = relation_instance.id();
        let _ = self.disconnect(relation_instance, behaviour_ty);
        self.relation_behaviour_storage.remove(&id, behaviour_ty);
        trace!("Removed relation behaviour {}", &behaviour_ty);
    }

    fn remove_behaviours(&self, relation_instance: ReactiveRelation) {
        self.relation_behaviour_storage.remove_all(&relation_instance.id());
    }

    fn remove_behaviours_by_key(&self, relation_instance_id: &RelationInstanceId) {
        self.relation_behaviour_storage.remove_all(relation_instance_id);
    }

    fn remove_behaviours_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) {
        self.relation_behaviour_storage.remove_by_behaviour(behaviour_ty);
        trace!("Removed all relation behaviours of type {}", &behaviour_ty);
    }

    fn has(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) -> bool {
        self.relation_behaviour_storage.has(&relation_instance.id(), behaviour_ty)
    }

    fn get_all(&self, relation_instance: ReactiveRelation) -> Vec<BehaviourTypeId> {
        self.relation_behaviour_storage.get_behaviours_by_instance(&relation_instance.id())
    }

    fn get_instances_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<ReactiveRelation> {
        self.relation_behaviour_storage.get_instances_by_behaviour(ty)
    }

    fn connect(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.relation_behaviour_storage.get(&relation_instance.id(), behaviour_ty) {
            return fsm.transition(BehaviourState::Connected);
        }
        Err(BehaviourTransitionError::BehaviourConnectFailed(BehaviourConnectFailed {}))
    }

    fn disconnect(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.relation_behaviour_storage.get(&relation_instance.id(), behaviour_ty) {
            return fsm.transition(BehaviourState::Ready);
        }
        Err(BehaviourTransitionError::BehaviourDisconnectFailed(BehaviourDisconnectFailed {}))
    }

    fn reconnect(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.relation_behaviour_storage.get(&relation_instance.id(), behaviour_ty) {
            return fsm.transition(BehaviourState::Ready).and_then(|_| fsm.transition(BehaviourState::Connected));
        }
        Err(BehaviourTransitionError::InvalidTransition)
    }
}

#[async_trait]
impl Lifecycle for RelationBehaviourManagerImpl {}
