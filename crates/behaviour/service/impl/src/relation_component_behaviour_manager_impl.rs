use async_trait::async_trait;
use log::trace;
use std::sync::Arc;

use inexor_rgf_behaviour_model_api::BehaviourConnectFailed;
use inexor_rgf_behaviour_model_api::BehaviourDisconnectFailed;
use inexor_rgf_behaviour_model_api::BehaviourState;
use inexor_rgf_behaviour_model_api::BehaviourTransitionError;
use inexor_rgf_behaviour_model_api::BehaviourTypeId;
use inexor_rgf_behaviour_model_api::ComponentBehaviourTypeId;
use inexor_rgf_behaviour_model_impl::RelationBehaviourStorage;
use inexor_rgf_behaviour_service_api::RelationComponentBehaviourManager;
use inexor_rgf_behaviour_service_api::RelationComponentBehaviourRegistry;
use inexor_rgf_graph::ComponentContainer;
use inexor_rgf_graph::RelationInstanceId;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_model_api::ReactiveInstance;
use inexor_rgf_reactive_model_impl::ReactiveRelation;
use springtime_di::component_alias;
use springtime_di::Component;

#[derive(Component)]
pub struct RelationComponentBehaviourManagerImpl {
    relation_component_behaviour_registry: Arc<dyn RelationComponentBehaviourRegistry + Send + Sync>,

    #[component(default = "RelationBehaviourStorage::new")]
    relation_behaviour_storage: RelationBehaviourStorage,
}

#[async_trait]
#[component_alias]
impl RelationComponentBehaviourManager for RelationComponentBehaviourManagerImpl {
    fn add_behaviours_to_relation(&self, relation_instance: ReactiveRelation) {
        let edge_key = relation_instance.id();
        for component_ty in relation_instance.get_components() {
            for factory in self.relation_component_behaviour_registry.get(&component_ty) {
                if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                    let behaviour_ty = behaviour.ty().clone();
                    self.relation_behaviour_storage.insert(edge_key.clone(), behaviour_ty.clone(), behaviour);
                    trace!("Added relation component behaviour {}", &behaviour_ty);
                }
            }
        }
    }

    fn add_behaviours_to_relation_component(&self, relation_instance: ReactiveRelation, component: inexor_rgf_graph::Component) {
        let edge_key = relation_instance.id();
        for factory in self.relation_component_behaviour_registry.get(&component.ty) {
            if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.relation_behaviour_storage.insert(edge_key.clone(), behaviour_ty.clone(), behaviour);
                trace!("Added relation component behaviour {}", &behaviour_ty);
            }
        }
    }

    fn add_behaviour_to_relation_component(&self, relation_instance: ReactiveRelation, component_behaviour_ty: &ComponentBehaviourTypeId) {
        let edge_key = relation_instance.id();
        for factory in self.relation_component_behaviour_registry.get(&component_behaviour_ty.component_ty) {
            if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.relation_behaviour_storage.insert(edge_key.clone(), behaviour_ty.clone(), behaviour);
                trace!("Added relation component behaviour {}", &behaviour_ty);
            }
        }
    }

    fn remove_behaviour_from_relation(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) {
        let edge_key = relation_instance.id();
        let _ = self.disconnect(relation_instance, behaviour_ty);
        self.relation_behaviour_storage.remove(&edge_key, behaviour_ty);
        trace!("Removed relation behaviour {}", &behaviour_ty);
    }

    fn remove_behaviours_from_relation(&self, relation_instance: ReactiveRelation) {
        self.relation_behaviour_storage.remove_all(&relation_instance.id());
    }

    fn remove_behaviours_from_relation_component(&self, relation_instance: ReactiveRelation, component: inexor_rgf_graph::Component) {
        let edge_key = relation_instance.id();
        for factory in self.relation_component_behaviour_registry.get(&component.ty) {
            self.relation_behaviour_storage.remove(&edge_key, factory.behaviour_ty());
            trace!("Removed relation component behaviour {}", factory.behaviour_ty());
        }
    }

    fn remove_behaviours_by_key(&self, edge_key: &RelationInstanceId) {
        self.relation_behaviour_storage.remove_all(edge_key);
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
impl Lifecycle for RelationComponentBehaviourManagerImpl {}
