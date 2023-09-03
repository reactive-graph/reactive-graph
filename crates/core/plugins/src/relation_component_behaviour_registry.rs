use std::sync::Arc;
use inexor_rgf_core_model::RelationInstanceId;

use crate::reactive::ComponentBehaviourTypeId;
use crate::reactive::ReactiveRelation;
use crate::behaviour::BehaviourFactory;

pub trait RelationComponentBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating relation component behaviours.
    /// If a relation instance has the given component then the given behaviour is applied.
    /// The behaviour will be created using the given RelationBehaviourCreator.
    #[allow(unused_variables)]
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>) {}

    /// Unregisters a factory for creating relation component behaviours.
    #[allow(unused_variables)]
    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {}
}
