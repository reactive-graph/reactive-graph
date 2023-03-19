use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;
use log::trace;

use crate::api::RelationComponentBehaviourManager;
use crate::api::RelationComponentBehaviourRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::BehaviourTypeId;
use crate::model::ComponentBehaviourTypeId;
use crate::model::ComponentContainer;
use crate::model::ReactiveRelationInstance;
use crate::reactive::BehaviourConnectFailed;
use crate::reactive::BehaviourDisconnectFailed;
use crate::reactive::BehaviourState;
use crate::reactive::BehaviourTransitionError;
use crate::reactive::RelationBehaviourStorage;

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
    fn add_behaviours_to_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
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

    fn add_behaviours_to_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: crate::model::Component) {
        let edge_key = relation_instance.get_key();
        for factory in self.relation_component_behaviour_registry.get(&component.ty) {
            if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.relation_behaviour_storage.0.insert(edge_key.clone(), behaviour_ty.clone(), behaviour);
                trace!("Added relation component behaviour {}", &behaviour_ty);
            }
        }
    }

    fn add_behaviour_to_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component_behaviour_ty: &ComponentBehaviourTypeId) {
        let edge_key = relation_instance.get_key();
        for factory in self.relation_component_behaviour_registry.get(&component_behaviour_ty.component_ty) {
            if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.relation_behaviour_storage.0.insert(edge_key.clone(), behaviour_ty.clone(), behaviour);
                trace!("Added relation component behaviour {}", &behaviour_ty);
            }
        }
    }

    fn remove_behaviour_from_relation(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) {
        let edge_key = relation_instance.get_key();
        let _ = self.disconnect(relation_instance, behaviour_ty);
        self.relation_behaviour_storage.0.remove(&edge_key, behaviour_ty);
        trace!("Removed relation behaviour {}", &behaviour_ty);
    }

    fn remove_behaviours_from_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        self.relation_behaviour_storage.0.remove_all(&relation_instance.get_key());
    }

    fn remove_behaviours_from_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: crate::model::Component) {
        let edge_key = relation_instance.get_key();
        for factory in self.relation_component_behaviour_registry.get(&component.ty) {
            self.relation_behaviour_storage.0.remove(&edge_key, factory.behaviour_ty());
            trace!("Removed relation component behaviour {}", factory.behaviour_ty());
        }
    }

    fn remove_behaviours_by_key(&self, edge_key: &EdgeKey) {
        self.relation_behaviour_storage.0.remove_all(edge_key);
    }

    fn remove_behaviours_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) {
        self.relation_behaviour_storage.0.remove_by_behaviour(behaviour_ty);
        trace!("Removed all relation behaviours of type {}", &behaviour_ty);
    }

    fn has(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) -> bool {
        self.relation_behaviour_storage.0.has(&relation_instance.get_key(), behaviour_ty)
    }

    fn get_all(&self, relation_instance: Arc<ReactiveRelationInstance>) -> Vec<BehaviourTypeId> {
        self.relation_behaviour_storage.0.get_behaviours_by_instance(&relation_instance.get_key())
    }

    fn get_instances_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<Arc<ReactiveRelationInstance>> {
        self.relation_behaviour_storage.0.get_instances_by_behaviour(ty)
    }

    fn connect(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.relation_behaviour_storage.0.get(&relation_instance.get_key(), behaviour_ty) {
            return fsm.transition(BehaviourState::Connected);
        }
        Err(BehaviourTransitionError::BehaviourConnectFailed(BehaviourConnectFailed {}))
    }

    fn disconnect(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.relation_behaviour_storage.0.get(&relation_instance.get_key(), behaviour_ty) {
            return fsm.transition(BehaviourState::Ready);
        }
        Err(BehaviourTransitionError::BehaviourDisconnectFailed(BehaviourDisconnectFailed {}))
    }

    fn reconnect(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.relation_behaviour_storage.0.get(&relation_instance.get_key(), behaviour_ty) {
            return fsm.transition(BehaviourState::Ready).and_then(|_| fsm.transition(BehaviourState::Connected));
        }
        Err(BehaviourTransitionError::InvalidTransition)
    }
}
