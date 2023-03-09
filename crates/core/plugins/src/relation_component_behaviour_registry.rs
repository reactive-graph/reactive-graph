use std::sync::Arc;

use crate::model::ComponentBehaviourTypeId;
use crate::model::ReactiveRelationInstance;
use crate::reactive::BehaviourFactory;

pub trait RelationComponentBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating relation component behaviours.
    /// If a relation instance has the given component then the given behaviour is applied.
    /// The behaviour will be created using the given RelationBehaviourCreator.
    #[allow(unused_variables)]
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>) {}

    /// Unregisters a factory for creating relation component behaviours.
    #[allow(unused_variables)]
    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {}
}
