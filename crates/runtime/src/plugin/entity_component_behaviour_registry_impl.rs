use async_trait::async_trait;
use std::sync::Arc;

use uuid::Uuid;

use inexor_rgf_behaviour_api::prelude::*;

use crate::plugins::EntityComponentBehaviourRegistry;
use crate::reactive::ReactiveEntity;

pub struct EntityComponentBehaviourRegistryImpl {
    entity_component_behaviour_manager: Arc<dyn crate::api::EntityComponentBehaviourManager>,
    entity_component_behaviour_registry: Arc<dyn crate::api::EntityComponentBehaviourRegistry>,
    reactive_entity_manager: Arc<dyn crate::api::ReactiveEntityManager>,
}

impl EntityComponentBehaviourRegistryImpl {
    pub fn new(
        entity_component_behaviour_manager: Arc<dyn crate::api::EntityComponentBehaviourManager>,
        entity_component_behaviour_registry: Arc<dyn crate::api::EntityComponentBehaviourRegistry>,
        reactive_entity_manager: Arc<dyn crate::api::ReactiveEntityManager>,
    ) -> Self {
        Self {
            entity_component_behaviour_manager,
            entity_component_behaviour_registry,
            reactive_entity_manager,
        }
    }
}

#[async_trait]
impl EntityComponentBehaviourRegistry for EntityComponentBehaviourRegistryImpl {
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
