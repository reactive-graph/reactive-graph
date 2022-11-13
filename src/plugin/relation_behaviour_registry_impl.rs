use std::sync::Arc;

use crate::model::ReactiveRelationInstance;
use crate::model::RelationBehaviourTypeId;
use crate::plugins::RelationBehaviourRegistry;
use crate::reactive::BehaviourFactory;

pub struct RelationBehaviourRegistryImpl {
    relation_behaviour_registry: Arc<dyn crate::api::RelationBehaviourRegistry>,
}

impl RelationBehaviourRegistryImpl {
    pub fn new(relation_behaviour_registry: Arc<dyn crate::api::RelationBehaviourRegistry>) -> Self {
        Self { relation_behaviour_registry }
    }
}

impl RelationBehaviourRegistry for RelationBehaviourRegistryImpl {
    fn register(&self, relation_behaviour_ty: RelationBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>) {
        self.relation_behaviour_registry.register(relation_behaviour_ty, factory);
    }

    fn unregister(&self, relation_behaviour_ty: &RelationBehaviourTypeId) {
        self.relation_behaviour_registry.unregister(relation_behaviour_ty);
    }
}
