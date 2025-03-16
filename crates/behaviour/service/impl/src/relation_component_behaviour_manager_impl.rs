use async_trait::async_trait;
use log::trace;
use std::sync::Arc;

use reactive_graph_behaviour_model_api::BehaviourConnectFailed;
use reactive_graph_behaviour_model_api::BehaviourDisconnectFailed;
use reactive_graph_behaviour_model_api::BehaviourState;
use reactive_graph_behaviour_model_api::BehaviourTransitionError;
use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_behaviour_model_api::ComponentBehaviourTypeId;
use reactive_graph_behaviour_model_impl::RelationBehaviourStorage;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourManager;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourRegistry;
use reactive_graph_graph::ComponentContainer;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_api::ReactiveInstance;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use springtime_di::Component;
use springtime_di::component_alias;

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
        let relation_instance_id = relation_instance.id();
        for component_ty in relation_instance.get_components() {
            for factory in self.relation_component_behaviour_registry.get(&component_ty) {
                if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                    let behaviour_ty = behaviour.ty().clone();
                    self.relation_behaviour_storage
                        .insert(relation_instance_id.clone(), behaviour_ty.clone(), behaviour);
                    trace!("Added relation component behaviour {}", &behaviour_ty);
                }
            }
        }
    }

    fn add_behaviours_to_relation_component(&self, relation_instance: ReactiveRelation, component: reactive_graph_graph::Component) {
        let relation_instance_id = relation_instance.id();
        for factory in self.relation_component_behaviour_registry.get(&component.ty) {
            if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.relation_behaviour_storage
                    .insert(relation_instance_id.clone(), behaviour_ty.clone(), behaviour);
                trace!("Added relation component behaviour {}", &behaviour_ty);
            }
        }
    }

    fn add_behaviour_to_relation_component(&self, relation_instance: ReactiveRelation, component_behaviour_ty: &ComponentBehaviourTypeId) {
        let relation_instance_id = relation_instance.id();
        for factory in self.relation_component_behaviour_registry.get(&component_behaviour_ty.component_ty) {
            if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.relation_behaviour_storage
                    .insert(relation_instance_id.clone(), behaviour_ty.clone(), behaviour);
                trace!("Added relation component behaviour {}", &behaviour_ty);
            }
        }
    }

    fn remove_behaviour_from_relation(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) {
        let relation_instance_id = relation_instance.id();
        let _ = self.disconnect(relation_instance, behaviour_ty);
        self.relation_behaviour_storage.remove(&relation_instance_id, behaviour_ty);
        trace!("Removed relation behaviour {}", &behaviour_ty);
    }

    fn remove_behaviours_from_relation(&self, relation_instance: ReactiveRelation) {
        self.relation_behaviour_storage.remove_all(&relation_instance.id());
    }

    fn remove_behaviours_from_relation_component(&self, relation_instance: ReactiveRelation, component: reactive_graph_graph::Component) {
        let relation_instance_id = relation_instance.id();
        for factory in self.relation_component_behaviour_registry.get(&component.ty) {
            self.relation_behaviour_storage.remove(&relation_instance_id, factory.behaviour_ty());
            trace!("Removed relation component behaviour {}", factory.behaviour_ty());
        }
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
impl Lifecycle for RelationComponentBehaviourManagerImpl {}
