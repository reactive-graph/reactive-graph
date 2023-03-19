use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;
use log::trace;

use crate::api::RelationBehaviourManager;
use crate::api::RelationBehaviourRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::BehaviourTypeId;
use crate::model::ReactiveRelationInstance;
use crate::reactive::BehaviourConnectFailed;
use crate::reactive::BehaviourDisconnectFailed;
use crate::reactive::BehaviourState;
use crate::reactive::BehaviourTransitionError;
use crate::reactive::RelationBehaviourStorage;

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
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        let relation_ty = relation_instance.relation_type_id();
        for factory in self.relation_behaviour_registry.get(&relation_ty) {
            if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                self.relation_behaviour_storage
                    .0
                    .insert(edge_key.clone(), behaviour.ty().clone(), behaviour.clone());
                trace!("Added relation behaviour {}", behaviour.ty());
            }
        }
    }

    fn add_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) {
        if let Some(factory) = self.relation_behaviour_registry.get_factory_by_behaviour_type(behaviour_ty) {
            let edge_key = relation_instance.get_key();
            if let Ok(behaviour) = factory.create(relation_instance) {
                let behaviour_ty = behaviour.ty().clone();
                self.relation_behaviour_storage.0.insert(edge_key.clone(), behaviour_ty.clone(), behaviour);
                trace!("Added relation behaviour {}", &behaviour_ty);
            }
        }
    }

    fn remove_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) {
        let edge_key = relation_instance.get_key();
        let _ = self.disconnect(relation_instance, behaviour_ty);
        self.relation_behaviour_storage.0.remove(&edge_key, behaviour_ty);
        trace!("Removed relation behaviour {}", &behaviour_ty);
    }

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        self.relation_behaviour_storage.0.remove_all(&relation_instance.get_key());
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
