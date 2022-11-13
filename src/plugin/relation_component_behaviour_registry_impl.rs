use std::sync::Arc;

use crate::model::ComponentBehaviourTypeId;
use crate::model::ReactiveRelationInstance;
use crate::plugins::RelationComponentBehaviourRegistry;
use crate::reactive::BehaviourFactory;

pub struct RelationComponentBehaviourRegistryImpl {
    relation_component_behaviour_registry: Arc<dyn crate::api::RelationComponentBehaviourRegistry>,
}

impl RelationComponentBehaviourRegistryImpl {
    pub fn new(relation_component_behaviour_registry: Arc<dyn crate::api::RelationComponentBehaviourRegistry>) -> Self {
        Self {
            relation_component_behaviour_registry,
        }
    }
}

impl RelationComponentBehaviourRegistry for RelationComponentBehaviourRegistryImpl {
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>) {
        self.relation_component_behaviour_registry.register(component_behaviour_ty, factory);
    }

    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        self.relation_component_behaviour_registry.unregister(component_behaviour_ty);
    }
}
