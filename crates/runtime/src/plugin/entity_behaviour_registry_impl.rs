use std::sync::Arc;

use crate::model::EntityBehaviourTypeId;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourRegistry;
use crate::reactive::BehaviourFactory;

pub struct EntityBehaviourRegistryImpl {
    entity_behaviour_manager: Arc<dyn crate::api::EntityBehaviourManager>,
    entity_behaviour_registry: Arc<dyn crate::api::EntityBehaviourRegistry>,
    reactive_entity_instance_manager: Arc<dyn crate::api::ReactiveEntityInstanceManager>,
}

impl EntityBehaviourRegistryImpl {
    pub fn new(
        entity_behaviour_manager: Arc<dyn crate::api::EntityBehaviourManager>,
        entity_behaviour_registry: Arc<dyn crate::api::EntityBehaviourRegistry>,
        reactive_entity_instance_manager: Arc<dyn crate::api::ReactiveEntityInstanceManager>,
    ) -> Self {
        Self {
            entity_behaviour_manager,
            entity_behaviour_registry,
            reactive_entity_instance_manager,
        }
    }
}

impl EntityBehaviourRegistry for EntityBehaviourRegistryImpl {
    fn register(&self, entity_behaviour_ty: EntityBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>) {
        self.entity_behaviour_registry.register(entity_behaviour_ty.clone(), factory);
        self.reactive_entity_instance_manager
            .add_behaviour_to_all_entity_instances(&entity_behaviour_ty);
    }

    fn unregister(&self, entity_behaviour_ty: &EntityBehaviourTypeId) {
        self.entity_behaviour_registry.unregister(entity_behaviour_ty);
        self.entity_behaviour_manager.remove_behaviours_by_behaviour(&entity_behaviour_ty.behaviour_ty);
    }
}
