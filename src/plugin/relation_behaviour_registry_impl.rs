use std::sync::Arc;

use crate::model::ReactiveRelationInstance;
use crate::model::RelationBehaviourTypeId;
use crate::plugins::RelationBehaviourRegistry;
use crate::reactive::BehaviourFactory;

pub struct RelationBehaviourRegistryImpl {
    relation_behaviour_manager: Arc<dyn crate::api::RelationBehaviourManager>,
    relation_behaviour_registry: Arc<dyn crate::api::RelationBehaviourRegistry>,
    reactive_relation_instance_manager: Arc<dyn crate::api::ReactiveRelationInstanceManager>,
}

impl RelationBehaviourRegistryImpl {
    pub fn new(
        relation_behaviour_manager: Arc<dyn crate::api::RelationBehaviourManager>,
        relation_behaviour_registry: Arc<dyn crate::api::RelationBehaviourRegistry>,
        reactive_relation_instance_manager: Arc<dyn crate::api::ReactiveRelationInstanceManager>,
    ) -> Self {
        Self {
            relation_behaviour_manager,
            relation_behaviour_registry,
            reactive_relation_instance_manager,
        }
    }
}

impl RelationBehaviourRegistry for RelationBehaviourRegistryImpl {
    fn register(&self, relation_behaviour_ty: RelationBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>) {
        self.relation_behaviour_registry.register(relation_behaviour_ty.clone(), factory);
        self.reactive_relation_instance_manager
            .add_behaviour_to_all_relation_instances(&relation_behaviour_ty);
    }

    fn unregister(&self, relation_behaviour_ty: &RelationBehaviourTypeId) {
        self.relation_behaviour_registry.unregister(relation_behaviour_ty);
        self.relation_behaviour_manager
            .remove_behaviours_by_behaviour(&relation_behaviour_ty.behaviour_ty);
    }
}
