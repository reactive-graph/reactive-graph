use async_trait::async_trait;
use std::sync::Arc;

use uuid::Uuid;

use reactive_graph_behaviour_model_api::prelude::*;

use reactive_graph_reactive_model_impl::ReactiveEntity;

pub struct EntityBehaviourRegistryDelegate {
    entity_behaviour_manager: Arc<dyn reactive_graph_behaviour_service_api::EntityBehaviourManager + Send + Sync>,
    entity_behaviour_registry: Arc<dyn reactive_graph_behaviour_service_api::EntityBehaviourRegistry + Send + Sync>,
    reactive_entity_manager: Arc<dyn reactive_graph_reactive_service_api::ReactiveEntityManager + Send + Sync>,
}

impl EntityBehaviourRegistryDelegate {
    pub fn new(
        entity_behaviour_manager: Arc<dyn reactive_graph_behaviour_service_api::EntityBehaviourManager + Send + Sync>,
        entity_behaviour_registry: Arc<dyn reactive_graph_behaviour_service_api::EntityBehaviourRegistry + Send + Sync>,
        reactive_entity_manager: Arc<dyn reactive_graph_reactive_service_api::ReactiveEntityManager + Send + Sync>,
    ) -> Self {
        Self {
            entity_behaviour_manager,
            entity_behaviour_registry,
            reactive_entity_manager,
        }
    }
}

#[async_trait]
impl reactive_graph_plugin_api::EntityBehaviourRegistry for EntityBehaviourRegistryDelegate {
    async fn register(&self, entity_behaviour_ty: EntityBehaviourTypeId, factory: Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>) {
        self.entity_behaviour_registry.register(entity_behaviour_ty.clone(), factory);
        self.reactive_entity_manager.add_behaviour_to_all_entity_instances(&entity_behaviour_ty);
    }

    async fn unregister(&self, entity_behaviour_ty: &EntityBehaviourTypeId) {
        self.entity_behaviour_registry.unregister(entity_behaviour_ty);
        self.entity_behaviour_manager.remove_behaviours_by_behaviour(&entity_behaviour_ty.behaviour_ty);
    }
}
