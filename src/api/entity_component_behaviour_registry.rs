use std::sync::Arc;

use async_trait::async_trait;

use crate::model::BehaviourTypeId;
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

    /// Returns all entity component behaviours.
    fn get_all(&self) -> Vec<ComponentBehaviourTypeId>;

    /// Returns the entity behaviour factories for the given component type.
    fn get(&self, component_ty: &ComponentTypeId) -> Vec<Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>>;

    /// Returns the component behaviours for the given component type.
    fn get_behaviour_types(&self, component_ty: &ComponentTypeId) -> Vec<ComponentBehaviourTypeId>;

    /// Returns the entity behaviour for the given behaviour type if the entity component behaviour exists.
    fn get_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<ComponentBehaviourTypeId>;

    /// Returns the count of entity component behaviours.
    fn count(&self) -> usize;
}
