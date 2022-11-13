use std::sync::Arc;

use async_trait::async_trait;

use crate::model::ComponentBehaviourTypeId;
use crate::model::ComponentTypeId;
use crate::model::ReactiveEntityInstance;
use crate::reactive::BehaviourFactory;

#[async_trait]
pub trait EntityComponentBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating entity component behaviours.
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>);

    /// Unregisters a factory for creating entity component behaviours.
    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId);

    /// Returns the entity behaviour factories for the given component type.
    fn get(&self, component_ty: &ComponentTypeId) -> Vec<Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>>;
}
