use std::sync::Arc;

use async_trait::async_trait;

use crate::model::ComponentBehaviourTypeId;
use crate::model::ComponentTypeId;
use crate::model::ReactiveRelationInstance;
use crate::reactive::BehaviourFactory;

#[async_trait]
pub trait RelationComponentBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating relation component behaviours.
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>);

    /// Unregisters a factory for creating relation component behaviours.
    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId);

    /// Returns the relation behaviour factories for the given component type.
    fn get(&self, component_ty: &ComponentTypeId) -> Vec<Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>>;
}
