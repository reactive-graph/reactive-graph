use std::sync::Arc;

use crate::model::ComponentBehaviourTypeId;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityComponentBehaviourRegistry;
use crate::reactive::BehaviourFactory;

pub struct EntityComponentBehaviourRegistryImpl {
    entity_component_behaviour_registry: Arc<dyn crate::api::EntityComponentBehaviourRegistry>,
}

impl EntityComponentBehaviourRegistryImpl {
    pub fn new(entity_component_behaviour_registry: Arc<dyn crate::api::EntityComponentBehaviourRegistry>) -> Self {
        Self {
            entity_component_behaviour_registry,
        }
    }
}

impl EntityComponentBehaviourRegistry for EntityComponentBehaviourRegistryImpl {
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>) {
        self.entity_component_behaviour_registry.register(component_behaviour_ty, factory);
    }

    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        self.entity_component_behaviour_registry.unregister(component_behaviour_ty);
    }
}
