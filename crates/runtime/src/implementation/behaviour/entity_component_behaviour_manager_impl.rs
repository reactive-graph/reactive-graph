use async_trait::async_trait;
use log::trace;
use uuid::Uuid;

use crate::api::EntityComponentBehaviourManager;
use crate::api::EntityComponentBehaviourRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::reactive::BehaviourTypeId;
use crate::reactive::ComponentBehaviourTypeId;
use crate::reactive::ComponentContainer;
use crate::reactive::ReactiveEntity;
use crate::behaviour::BehaviourConnectFailed;
use crate::behaviour::BehaviourDisconnectFailed;
use crate::behaviour::BehaviourState;
use crate::behaviour::BehaviourTransitionError;
use crate::behaviour::EntityBehaviourStorage;

#[wrapper]
pub struct EntityComponentBehaviourStorageWrapper(EntityBehaviourStorage);

#[provides]
fn create_behaviour_providers() -> EntityComponentBehaviourStorageWrapper {
    EntityComponentBehaviourStorageWrapper(EntityBehaviourStorage::new())
}

#[component]
pub struct EntityComponentBehaviourManagerImpl {
    entity_component_behaviour_registry: Wrc<dyn EntityComponentBehaviourRegistry>,

    entity_behaviour_storage: EntityComponentBehaviourStorageWrapper,
}

#[async_trait]
#[provides]
impl EntityComponentBehaviourManager for EntityComponentBehaviourManagerImpl {
    fn add_behaviours_to_entity(&self, entity_instance: ReactiveEntity) {
        for component_ty in entity_instance.get_components() {
            for factory in self.entity_component_behaviour_registry.get(&component_ty) {
                if let Ok(behaviour) = factory.create(entity_instance.clone()) {
                    let behaviour_ty = behaviour.ty().clone();
                    self.entity_behaviour_storage.0.insert(entity_instance.id, behaviour.ty().clone(), behaviour);
                    trace!("Added entity component behaviour {}", &behaviour_ty);
                }
            }
        }
    }

    fn add_behaviours_to_entity_component(&self, entity_instance: ReactiveEntity, component: crate::model::Component) {
        for factory in self.entity_component_behaviour_registry.get(&component.ty) {
            if let Ok(behaviour) = factory.create(entity_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.entity_behaviour_storage.0.insert(entity_instance.id, behaviour_ty.clone(), behaviour);
                trace!("Added entity component behaviour {}", &behaviour_ty);
            }
        }
    }

    fn add_behaviour_to_entity_component(&self, entity_instance: ReactiveEntity, component_behaviour_ty: &ComponentBehaviourTypeId) {
        for factory in self.entity_component_behaviour_registry.get(&component_behaviour_ty.component_ty) {
            if let Ok(behaviour) = factory.create(entity_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.entity_behaviour_storage.0.insert(entity_instance.id, behaviour_ty.clone(), behaviour);
                trace!("Added entity component behaviour {}", &behaviour_ty);
            }
        }
    }

    fn remove_behaviour_from_entity(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) {
        let id = entity_instance.id;
        let _ = self.disconnect(entity_instance, behaviour_ty);
        self.entity_behaviour_storage.0.remove(&id, behaviour_ty);
        trace!("Removed entity behaviour {} from {}", &behaviour_ty, id);
    }

    fn remove_behaviours_from_entity(&self, entity_instance: ReactiveEntity) {
        self.entity_behaviour_storage.0.remove_all(&entity_instance.id);
    }

    fn remove_behaviours_from_entity_component(&self, entity_instance: ReactiveEntity, component: crate::model::Component) {
        for factory in self.entity_component_behaviour_registry.get(&component.ty) {
            self.entity_behaviour_storage.0.remove(&entity_instance.id, factory.behaviour_ty());
            trace!("Removed entity component behaviour {}", factory.behaviour_ty());
        }
    }

    fn remove_behaviours_by_id(&self, id: &Uuid) {
        self.entity_behaviour_storage.0.remove_all(id);
    }

    fn remove_behaviours_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) {
        self.entity_behaviour_storage.0.remove_by_behaviour(behaviour_ty);
        trace!("Removed all entity component behaviours of type {}", &behaviour_ty);
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
