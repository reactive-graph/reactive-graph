use std::sync::Arc;

use crate::model::EntityBehaviourTypeId;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourRegistry;
use crate::reactive::BehaviourFactory;

pub struct EntityBehaviourRegistryImpl {
    entity_behaviour_registry: Arc<dyn crate::api::EntityBehaviourRegistry>,
}

impl EntityBehaviourRegistryImpl {
    pub fn new(entity_behaviour_registry: Arc<dyn crate::api::EntityBehaviourRegistry>) -> Self {
        Self { entity_behaviour_registry }
    }
}

impl EntityBehaviourRegistry for EntityBehaviourRegistryImpl {
    fn register(&self, entity_behaviour_ty: EntityBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>) {
        self.entity_behaviour_registry.register(entity_behaviour_ty.clone(), factory);
    }

    fn unregister(&self, entity_behaviour_ty: &EntityBehaviourTypeId) {
        self.entity_behaviour_registry.unregister(entity_behaviour_ty);
    }
}
