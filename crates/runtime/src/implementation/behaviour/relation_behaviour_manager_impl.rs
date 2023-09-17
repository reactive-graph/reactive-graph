use async_trait::async_trait;
use log::trace;

use crate::api::RelationBehaviourManager;
use crate::api::RelationBehaviourRegistry;
use crate::behaviour::BehaviourConnectFailed;
use crate::behaviour::BehaviourDisconnectFailed;
use crate::behaviour::BehaviourState;
use crate::behaviour::BehaviourTransitionError;
use crate::behaviour::RelationBehaviourStorage;
use crate::behaviour_api::BehaviourTypeId;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::RelationInstanceId;
use crate::reactive::ReactiveRelation;
use inexor_rgf_reactive_api::prelude::*;

#[wrapper]
pub struct RelationBehaviourStorageWrapper(RelationBehaviourStorage);

#[provides]
fn create_relation_behaviour_providers() -> RelationBehaviourStorageWrapper {
    RelationBehaviourStorageWrapper(RelationBehaviourStorage::new())
}

#[component]
pub struct RelationBehaviourManagerImpl {
    relation_behaviour_registry: Wrc<dyn RelationBehaviourRegistry>,

    relation_behaviour_storage: RelationBehaviourStorageWrapper,
}

#[async_trait]
#[provides]
impl RelationBehaviourManager for RelationBehaviourManagerImpl {
    fn add_behaviours(&self, relation_instance: ReactiveRelation) {
        let id = relation_instance.id();
        let relation_ty = relation_instance.relation_type_id();
        for factory in self.relation_behaviour_registry.get(&relation_ty) {
            if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                self.relation_behaviour_storage.0.insert(id.clone(), behaviour.ty().clone(), behaviour.clone());
                trace!("Added relation behaviour {}", behaviour.ty());
            }
        }
    }

    fn add_behaviour(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) {
        if let Some(factory) = self.relation_behaviour_registry.get_factory_by_behaviour_type(behaviour_ty) {
            let id = relation_instance.id();
            if let Ok(behaviour) = factory.create(relation_instance) {
                let behaviour_ty = behaviour.ty().clone();
                self.relation_behaviour_storage.0.insert(id, behaviour_ty.clone(), behaviour);
                trace!("Added relation behaviour {}", &behaviour_ty);
            }
        }
    }

    fn remove_behaviour(&self, relation_instance: ReactiveRelation, behaviour_ty: &BehaviourTypeId) {
        let id = relation_instance.id();
        let _ = self.disconnect(relation_instance, behaviour_ty);
        self.relation_behaviour_storage.0.remove(&id, behaviour_ty);
        trace!("Removed relation behaviour {}", &behaviour_ty);
    }

    fn remove_behaviours(&self, relation_instance: ReactiveRelation) {
        self.relation_behaviour_storage.0.remove_all(&relation_instance.id());
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
