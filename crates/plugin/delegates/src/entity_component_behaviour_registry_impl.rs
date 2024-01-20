use async_trait::async_trait;
use std::sync::Arc;

use uuid::Uuid;

use inexor_rgf_behaviour_model_api::prelude::*;

use inexor_rgf_reactive_model_impl::ReactiveEntity;

pub struct EntityComponentBehaviourRegistryDelegate {
    entity_component_behaviour_manager: Arc<dyn inexor_rgf_behaviour_service_api::EntityComponentBehaviourManager + Send + Sync>,
    entity_component_behaviour_registry: Arc<dyn inexor_rgf_behaviour_service_api::EntityComponentBehaviourRegistry + Send + Sync>,
    reactive_entity_manager: Arc<dyn inexor_rgf_reactive_service_api::ReactiveEntityManager + Send + Sync>,
}

impl EntityComponentBehaviourRegistryDelegate {
    pub fn new(
        entity_component_behaviour_manager: Arc<dyn inexor_rgf_behaviour_service_api::EntityComponentBehaviourManager + Send + Sync>,
        entity_component_behaviour_registry: Arc<dyn inexor_rgf_behaviour_service_api::EntityComponentBehaviourRegistry + Send + Sync>,
        reactive_entity_manager: Arc<dyn inexor_rgf_reactive_service_api::ReactiveEntityManager + Send + Sync>,
    ) -> Self {
        Self {
            entity_component_behaviour_manager,
            entity_component_behaviour_registry,
            reactive_entity_manager,
        }
    }
}

#[async_trait]
impl inexor_rgf_plugin_api::EntityComponentBehaviourRegistry for EntityComponentBehaviourRegistryDelegate {
    async fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>) {
        self.entity_component_behaviour_registry.register(component_behaviour_ty.clone(), factory);
        self.reactive_entity_manager.add_behaviour_to_all_entity_components(&component_behaviour_ty);
    }

    async fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        self.entity_component_behaviour_registry.unregister(component_behaviour_ty);
        self.entity_component_behaviour_manager
            .remove_behaviours_by_behaviour(&component_behaviour_ty.behaviour_ty);
    }
}
