use async_trait::async_trait;
use log::trace;
use uuid::Uuid;

use crate::api::EntityBehaviourManager;
use crate::api::EntityBehaviourRegistry;
use crate::behaviour::BehaviourConnectFailed;
use crate::behaviour::BehaviourDisconnectFailed;
use crate::behaviour::BehaviourState;
use crate::behaviour::BehaviourTransitionError;
use crate::behaviour::EntityBehaviourStorage;
use crate::behaviour_api::BehaviourTypeId;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::reactive::ReactiveEntity;

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
    fn add_behaviours(&self, entity_instance: ReactiveEntity) {
        for factory in self.entity_behaviour_registry.get(&entity_instance.ty) {
            if let Ok(behaviour) = factory.create(entity_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.entity_behaviour_storage.0.insert(entity_instance.id, behaviour_ty.clone(), behaviour);
                trace!("Added entity behaviour {} to {}", &behaviour_ty, entity_instance.id);
            }
        }
    }

    fn add_behaviour(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) {
        if let Some(factory) = self.entity_behaviour_registry.get_factory_by_behaviour_type(behaviour_ty) {
            if let Ok(behaviour) = factory.create(entity_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.entity_behaviour_storage.0.insert(entity_instance.id, behaviour_ty.clone(), behaviour);
                trace!("Added entity behaviour {} to {}", &behaviour_ty, entity_instance.id);
            }
        }
    }

    fn remove_behaviour(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) {
        let id = entity_instance.id;
        let _ = self.disconnect(entity_instance, behaviour_ty);
        self.entity_behaviour_storage.0.remove(&id, behaviour_ty);
        trace!("Removed entity behaviour {} from {}", &behaviour_ty, id);
    }

    fn remove_behaviours(&self, entity_instance: ReactiveEntity) {
        self.entity_behaviour_storage.0.remove_all(&entity_instance.id);
    }

    fn remove_behaviours_by_id(&self, id: &Uuid) {
        self.entity_behaviour_storage.0.remove_all(id);
    }

    fn remove_behaviours_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) {
        self.entity_behaviour_storage.0.remove_by_behaviour(behaviour_ty);
        trace!("Removed all entity behaviours of type {}", &behaviour_ty);
    }

    fn has(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) -> bool {
        self.entity_behaviour_storage.0.has(&entity_instance.id, behaviour_ty)
    }

    fn get_all(&self, entity_instance: ReactiveEntity) -> Vec<BehaviourTypeId> {
        self.entity_behaviour_storage.0.get_behaviours_by_instance(&entity_instance.id)
    }

    fn get_instances_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<ReactiveEntity> {
        self.entity_behaviour_storage.0.get_instances_by_behaviour(ty)
    }

    fn connect(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.entity_behaviour_storage.0.get(&entity_instance.id, behaviour_ty) {
            return fsm.transition(BehaviourState::Connected);
        }
        Err(BehaviourTransitionError::BehaviourConnectFailed(BehaviourConnectFailed {}))
    }

    fn disconnect(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.entity_behaviour_storage.0.get(&entity_instance.id, behaviour_ty) {
            return fsm.transition(BehaviourState::Ready);
        }
        Err(BehaviourTransitionError::BehaviourDisconnectFailed(BehaviourDisconnectFailed {}))
    }

    fn reconnect(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.entity_behaviour_storage.0.get(&entity_instance.id, behaviour_ty) {
            return fsm.transition(BehaviourState::Ready).and_then(|_| fsm.transition(BehaviourState::Connected));
        }
        Err(BehaviourTransitionError::InvalidTransition)
    }
}
