use std::sync::Arc;
use inexor_rgf_core_model::RelationInstanceId;

use crate::reactive::ReactiveRelation;
use crate::reactive::RelationBehaviourTypeId;
use crate::behaviour::BehaviourFactory;

pub trait RelationBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating relation behaviours.
    /// If a relation instance is of the relation type then the given behaviour is applied.
    /// The behaviour will be created using the given RelationBehaviourCreator.
    #[allow(unused_variables)]
    fn register(&self, relation_behaviour_ty: RelationBehaviourTypeId, factory: Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>) {}

    /// Unregisters a factory for creating relation behaviours.
    #[allow(unused_variables)]
    fn unregister(&self, relation_behaviour_ty: &RelationBehaviourTypeId) {}
}
