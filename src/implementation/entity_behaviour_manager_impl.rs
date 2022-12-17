use std::sync::Arc;

use async_trait::async_trait;
use log::trace;
use uuid::Uuid;

use crate::api::EntityBehaviourManager;
use crate::api::EntityBehaviourRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::BehaviourTypeId;
use crate::model::ReactiveEntityInstance;
use crate::reactive::BehaviourConnectFailed;
use crate::reactive::BehaviourDisconnectFailed;
use crate::reactive::BehaviourState;
use crate::reactive::BehaviourTransitionError;
use crate::reactive::EntityBehaviourStorage;

#[wrapper]
pub struct EntityBehaviourStorageWrapper(EntityBehaviourStorage);

#[provides]
fn create_entity_behaviour_storage() -> EntityBehaviourStorageWrapper {
    EntityBehaviourStorageWrapper(EntityBehaviourStorage::new())
}

#[component]
pub struct EntityBehaviourManagerImpl {
    entity_behaviour_registry: Wrc<dyn EntityBehaviourRegistry>,

    entity_behaviour_storage: EntityBehaviourStorageWrapper,
}

#[async_trait]
#[provides]
impl EntityBehaviourManager for EntityBehaviourManagerImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        for factory in self.entity_behaviour_registry.get(&entity_instance.ty) {
            if let Ok(behaviour) = factory.create(entity_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.entity_behaviour_storage.0.insert(entity_instance.id, behaviour_ty.clone(), behaviour);
                trace!("Added entity behaviour {}", &behaviour_ty);
            }
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.entity_behaviour_storage.0.remove_all(&entity_instance.id);
    }

    fn remove_behaviours_by_id(&self, id: &Uuid) {
        self.entity_behaviour_storage.0.remove_all(id);
    }

    fn has(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> bool {
        self.entity_behaviour_storage.0.has(&entity_instance.id, behaviour_ty)
    }

    fn get_all(&self, entity_instance: Arc<ReactiveEntityInstance>) -> Vec<BehaviourTypeId> {
        self.entity_behaviour_storage.0.get_behaviours_by_instance(&entity_instance.id)
    }

    fn get_instances_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<Arc<ReactiveEntityInstance>> {
        self.entity_behaviour_storage.0.get_instances_by_behaviour(&ty)
    }

    fn connect(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.entity_behaviour_storage.0.get(&entity_instance.id, behaviour_ty) {
            return fsm.transition(BehaviourState::Connected);
        }
        Err(BehaviourTransitionError::BehaviourConnectFailed(BehaviourConnectFailed {}))
    }

    fn disconnect(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.entity_behaviour_storage.0.get(&entity_instance.id, behaviour_ty) {
            return fsm.transition(BehaviourState::Ready);
        }
        Err(BehaviourTransitionError::BehaviourDisconnectFailed(BehaviourDisconnectFailed {}))
    }

    fn reconnect(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.entity_behaviour_storage.0.get(&entity_instance.id, behaviour_ty) {
            return fsm.transition(BehaviourState::Ready).and_then(|_| fsm.transition(BehaviourState::Connected));
        }
        Err(BehaviourTransitionError::InvalidTransition)
    }
}
