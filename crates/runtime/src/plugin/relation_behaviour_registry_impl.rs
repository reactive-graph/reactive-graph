use std::sync::Arc;
use inexor_rgf_core_model::RelationInstanceId;

use crate::reactive::ReactiveRelation;
use crate::reactive::RelationBehaviourTypeId;
use crate::plugins::RelationBehaviourRegistry;
use crate::behaviour::BehaviourFactory;

pub struct RelationBehaviourRegistryImpl {
    relation_behaviour_manager: Arc<dyn crate::api::RelationBehaviourManager>,
    relation_behaviour_registry: Arc<dyn crate::api::RelationBehaviourRegistry>,
    reactive_relation_manager: Arc<dyn crate::api::ReactiveRelationManager>,
}

impl RelationBehaviourRegistryImpl {
    pub fn new(
        relation_behaviour_manager: Arc<dyn crate::api::RelationBehaviourManager>,
        relation_behaviour_registry: Arc<dyn crate::api::RelationBehaviourRegistry>,
        reactive_relation_manager: Arc<dyn crate::api::ReactiveRelationManager>,
    ) -> Self {
        Self {
            relation_behaviour_manager,
            relation_behaviour_registry,
            reactive_relation_manager,
        }
    }
}

impl RelationBehaviourRegistry for RelationBehaviourRegistryImpl {
    fn register(&self, relation_behaviour_ty: RelationBehaviourTypeId, factory: Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>) {
        self.relation_behaviour_registry.register(relation_behaviour_ty.clone(), factory);
        self.reactive_relation_manager
            .add_behaviour_to_all_relation_instances(&relation_behaviour_ty);
    }

    fn unregister(&self, relation_behaviour_ty: &RelationBehaviourTypeId) {
        self.relation_behaviour_registry.unregister(relation_behaviour_ty);
        self.relation_behaviour_manager
            .remove_behaviours_by_behaviour(&relation_behaviour_ty.behaviour_ty);
    }
}
