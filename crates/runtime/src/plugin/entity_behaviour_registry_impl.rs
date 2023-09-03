use std::sync::Arc;
use uuid::Uuid;

use crate::reactive::EntityBehaviourTypeId;
use crate::reactive::ReactiveEntity;
use crate::plugins::EntityBehaviourRegistry;
use crate::behaviour::BehaviourFactory;

pub struct EntityBehaviourRegistryImpl {
    entity_behaviour_manager: Arc<dyn crate::api::EntityBehaviourManager>,
    entity_behaviour_registry: Arc<dyn crate::api::EntityBehaviourRegistry>,
    reactive_entity_manager: Arc<dyn crate::api::ReactiveEntityManager>,
}

impl EntityBehaviourRegistryImpl {
    pub fn new(
        entity_behaviour_manager: Arc<dyn crate::api::EntityBehaviourManager>,
        entity_behaviour_registry: Arc<dyn crate::api::EntityBehaviourRegistry>,
        reactive_entity_manager: Arc<dyn crate::api::ReactiveEntityManager>,
    ) -> Self {
        Self {
            entity_behaviour_manager,
            entity_behaviour_registry,
            reactive_entity_manager,
        }
    }
}

impl EntityBehaviourRegistry for EntityBehaviourRegistryImpl {
    fn register(&self, entity_behaviour_ty: EntityBehaviourTypeId, factory: Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>) {
        self.entity_behaviour_registry.register(entity_behaviour_ty.clone(), factory);
        self.reactive_entity_manager
            .add_behaviour_to_all_entity_instances(&entity_behaviour_ty);
    }

    fn unregister(&self, entity_behaviour_ty: &EntityBehaviourTypeId) {
        self.entity_behaviour_registry.unregister(entity_behaviour_ty);
        self.entity_behaviour_manager.remove_behaviours_by_behaviour(&entity_behaviour_ty.behaviour_ty);
    }
}
