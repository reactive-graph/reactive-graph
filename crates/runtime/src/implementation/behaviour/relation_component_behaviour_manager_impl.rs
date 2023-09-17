use async_trait::async_trait;
use log::trace;

use crate::api::RelationComponentBehaviourManager;
use crate::api::RelationComponentBehaviourRegistry;
use crate::behaviour::BehaviourConnectFailed;
use crate::behaviour::BehaviourDisconnectFailed;
use crate::behaviour::BehaviourState;
use crate::behaviour::BehaviourTransitionError;
use crate::behaviour::RelationBehaviourStorage;
use crate::behaviour_api::BehaviourTypeId;
use crate::behaviour_api::ComponentBehaviourTypeId;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::RelationInstanceId;
use crate::reactive::ReactiveRelation;
use inexor_rgf_reactive_api::prelude::*;

#[wrapper]
pub struct RelationComponentBehaviourStorageWrapper(RelationBehaviourStorage);

#[provides]
fn create_behaviour_providers() -> RelationComponentBehaviourStorageWrapper {
    RelationComponentBehaviourStorageWrapper(RelationBehaviourStorage::new())
}

#[component]
pub struct RelationComponentBehaviourManagerImpl {
    relation_component_behaviour_registry: Wrc<dyn RelationComponentBehaviourRegistry>,

    relation_behaviour_storage: RelationComponentBehaviourStorageWrapper,
}

#[async_trait]
#[provides]
impl RelationComponentBehaviourManager for RelationComponentBehaviourManagerImpl {
    fn add_behaviours_to_relation(&self, relation_instance: ReactiveRelation) {
        let edge_key = relation_instance.id();
        for component_ty in relation_instance.get_components() {
            for factory in self.relation_component_behaviour_registry.get(&component_ty) {
                if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                    let behaviour_ty = behaviour.ty().clone();
                    self.relation_behaviour_storage.0.insert(edge_key.clone(), behaviour_ty.clone(), behaviour);
                    trace!("Added relation component behaviour {}", &behaviour_ty);
                }
            }
        }
    }

    fn add_behaviours_to_relation_component(&self, relation_instance: ReactiveRelation, component: crate::model::Component) {
        let edge_key = relation_instance.id();
        for factory in self.relation_component_behaviour_registry.get(&component.ty) {
            if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.relation_behaviour_storage.0.insert(edge_key.clone(), behaviour_ty.clone(), behaviour);
                trace!("Added relation component behaviour {}", &behaviour_ty);
            }
        }
    }

    fn add_behaviour_to_relation_component(&self, relation_instance: ReactiveRelation, component_behaviour_ty: &ComponentBehaviourTypeId) {
        let edge_key = relation_instance.id();
        for factory in self.relation_component_behaviour_registry.get(&component_behaviour_ty.component_ty) {
            if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.relation_behaviour_storage.0.insert(edge_key.clone(), behaviour_ty.clone(), behaviour);
                trace!("Added relation component behaviour {}", &behaviour_ty);
            }
        }
    }

    fn remove_behaviour_from_relation(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) {
        let edge_key = relation_instance.id();
        let _ = self.disconnect(relation_instance, behaviour_ty);
        self.relation_behaviour_storage.0.remove(&edge_key, behaviour_ty);
        trace!("Removed relation behaviour {}", &behaviour_ty);
    }

    fn remove_behaviours_from_relation(&self, relation_instance: ReactiveRelation) {
        self.relation_behaviour_storage.0.remove_all(&relation_instance.id());
    }

    fn remove_behaviours_from_relation_component(&self, relation_instance: ReactiveRelation, component: crate::model::Component) {
        let edge_key = relation_instance.id();
        for factory in self.relation_component_behaviour_registry.get(&component.ty) {
            self.relation_behaviour_storage.0.remove(&edge_key, factory.behaviour_ty());
            trace!("Removed relation component behaviour {}", factory.behaviour_ty());
        }
    }

    fn remove_behaviours_by_key(&self, edge_key: &RelationInstanceId) {
        self.relation_behaviour_storage.0.remove_all(edge_key);
    }

    fn remove_behaviours_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) {
        self.relation_behaviour_storage.0.remove_by_behaviour(behaviour_ty);
        trace!("Removed all relation behaviours of type {}", &behaviour_ty);
    }

    fn has(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) -> bool {
        self.relation_behaviour_storage.0.has(&relation_instance.id(), behaviour_ty)
    }

    fn get_all(&self, relation_instance: ReactiveRelation) -> Vec<BehaviourTypeId> {
        self.relation_behaviour_storage.0.get_behaviours_by_instance(&relation_instance.id())
    }

    fn get_instances_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<ReactiveRelation> {
        self.relation_behaviour_storage.0.get_instances_by_behaviour(ty)
    }

    fn connect(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.relation_behaviour_storage.0.get(&relation_instance.id(), behaviour_ty) {
            return fsm.transition(BehaviourState::Connected);
        }
        Err(BehaviourTransitionError::BehaviourConnectFailed(BehaviourConnectFailed {}))
    }

    fn disconnect(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.relation_behaviour_storage.0.get(&relation_instance.id(), behaviour_ty) {
            return fsm.transition(BehaviourState::Ready);
        }
        Err(BehaviourTransitionError::BehaviourDisconnectFailed(BehaviourDisconnectFailed {}))
    }

    fn reconnect(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.relation_behaviour_storage.0.get(&relation_instance.id(), behaviour_ty) {
            return fsm.transition(BehaviourState::Ready).and_then(|_| fsm.transition(BehaviourState::Connected));
        }
        Err(BehaviourTransitionError::InvalidTransition)
    }
}
