use std::sync::Arc;

use async_trait::async_trait;
use log::trace;
use springtime_di::component_alias;
use springtime_di::Component;
use uuid::Uuid;

use inexor_rgf_behaviour_model_api::BehaviourConnectFailed;
use inexor_rgf_behaviour_model_api::BehaviourDisconnectFailed;
use inexor_rgf_behaviour_model_api::BehaviourState;
use inexor_rgf_behaviour_model_api::BehaviourTransitionError;
use inexor_rgf_behaviour_model_api::BehaviourTypeId;
use inexor_rgf_behaviour_model_impl::EntityBehaviourStorage;
use inexor_rgf_behaviour_service_api::EntityBehaviourManager;
use inexor_rgf_behaviour_service_api::EntityBehaviourRegistry;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_model_impl::ReactiveEntity;

#[derive(Component)]
pub struct EntityBehaviourManagerImpl {
    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,

    #[component(default = "EntityBehaviourStorage::new")]
    entity_behaviour_storage: EntityBehaviourStorage,
}

#[async_trait]
#[component_alias]
impl EntityBehaviourManager for EntityBehaviourManagerImpl {
    fn add_behaviours(&self, entity_instance: ReactiveEntity) {
        for factory in self.entity_behaviour_registry.get(&entity_instance.ty) {
            if let Ok(behaviour) = factory.create(entity_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.entity_behaviour_storage.insert(entity_instance.id, behaviour_ty.clone(), behaviour);
                trace!("Added entity behaviour {} to {}", &behaviour_ty, entity_instance.id);
            }
        }
    }

    fn add_behaviour(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) {
        if let Some(factory) = self.entity_behaviour_registry.get_factory_by_behaviour_type(behaviour_ty) {
            if let Ok(behaviour) = factory.create(entity_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.entity_behaviour_storage.insert(entity_instance.id, behaviour_ty.clone(), behaviour);
                trace!("Added entity behaviour {} to {}", &behaviour_ty, entity_instance.id);
            }
        }
    }

    fn remove_behaviour(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) {
        let id = entity_instance.id;
        let _ = self.disconnect(entity_instance, behaviour_ty);
        self.entity_behaviour_storage.remove(&id, behaviour_ty);
        trace!("Removed entity behaviour {} from {}", &behaviour_ty, id);
    }

    fn remove_behaviours(&self, entity_instance: ReactiveEntity) {
        self.entity_behaviour_storage.remove_all(&entity_instance.id);
    }

    fn remove_behaviours_by_id(&self, id: &Uuid) {
        self.entity_behaviour_storage.remove_all(id);
    }

    fn remove_behaviours_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) {
        self.entity_behaviour_storage.remove_by_behaviour(behaviour_ty);
        trace!("Removed all entity behaviours of type {}", &behaviour_ty);
    }

    fn has(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) -> bool {
        self.entity_behaviour_storage.has(&entity_instance.id, behaviour_ty)
    }

    fn get_all(&self, entity_instance: ReactiveEntity) -> Vec<BehaviourTypeId> {
        self.entity_behaviour_storage.get_behaviours_by_instance(&entity_instance.id)
    }

    fn get_instances_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<ReactiveEntity> {
        self.entity_behaviour_storage.get_instances_by_behaviour(ty)
    }

    fn connect(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.entity_behaviour_storage.get(&entity_instance.id, behaviour_ty) {
            return fsm.transition(BehaviourState::Connected);
        }
        Err(BehaviourTransitionError::BehaviourConnectFailed(BehaviourConnectFailed {}))
    }

    fn disconnect(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.entity_behaviour_storage.get(&entity_instance.id, behaviour_ty) {
            return fsm.transition(BehaviourState::Ready);
        }
        Err(BehaviourTransitionError::BehaviourDisconnectFailed(BehaviourDisconnectFailed {}))
    }

    fn reconnect(&self, entity_instance: ReactiveEntity, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError> {
        if let Some(fsm) = self.entity_behaviour_storage.get(&entity_instance.id, behaviour_ty) {
            return fsm.transition(BehaviourState::Ready).and_then(|_| fsm.transition(BehaviourState::Connected));
        }
        Err(BehaviourTransitionError::InvalidTransition)
    }
}

#[async_trait]
impl Lifecycle for EntityBehaviourManagerImpl {}
