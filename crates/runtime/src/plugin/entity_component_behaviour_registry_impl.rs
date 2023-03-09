use std::sync::Arc;

use crate::model::ComponentBehaviourTypeId;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityComponentBehaviourRegistry;
use crate::reactive::BehaviourFactory;

pub struct EntityComponentBehaviourRegistryImpl {
    entity_component_behaviour_manager: Arc<dyn crate::api::EntityComponentBehaviourManager>,
    entity_component_behaviour_registry: Arc<dyn crate::api::EntityComponentBehaviourRegistry>,
    reactive_entity_instance_manager: Arc<dyn crate::api::ReactiveEntityInstanceManager>,
}

impl EntityComponentBehaviourRegistryImpl {
    pub fn new(
        entity_component_behaviour_manager: Arc<dyn crate::api::EntityComponentBehaviourManager>,
        entity_component_behaviour_registry: Arc<dyn crate::api::EntityComponentBehaviourRegistry>,
        reactive_entity_instance_manager: Arc<dyn crate::api::ReactiveEntityInstanceManager>,
    ) -> Self {
        Self {
            entity_component_behaviour_manager,
            entity_component_behaviour_registry,
            reactive_entity_instance_manager,
        }
    }
}

impl EntityComponentBehaviourRegistry for EntityComponentBehaviourRegistryImpl {
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>) {
        self.entity_component_behaviour_registry.register(component_behaviour_ty.clone(), factory);
        self.reactive_entity_instance_manager
            .add_behaviour_to_all_entity_components(&component_behaviour_ty);
    }

    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        self.entity_component_behaviour_registry.unregister(component_behaviour_ty);
        self.entity_component_behaviour_manager
            .remove_behaviours_by_behaviour(&component_behaviour_ty.behaviour_ty);
    }
}
