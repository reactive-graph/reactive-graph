use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;
use uuid::Uuid;

use inexor_rgf_behaviour_model_api::prelude::*;

use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_model_impl::ReactiveEntity;

#[injectable]
#[async_trait]
pub trait EntityComponentBehaviourRegistry: Send + Sync + Lifecycle {
    /// Registers a factory for creating entity component behaviours.
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>);

    /// Unregisters a factory for creating entity component behaviours.
    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId);

    /// Returns all entity component behaviours.
    fn get_all(&self) -> Vec<ComponentBehaviourTypeId>;

    /// Returns the entity behaviour factories for the given component type.
    fn get(&self, component_ty: &ComponentTypeId) -> Vec<Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>>;

    /// Returns the component behaviours for the given component type.
    fn get_behaviour_types(&self, component_ty: &ComponentTypeId) -> Vec<ComponentBehaviourTypeId>;

    /// Returns the entity behaviour for the given behaviour type if the entity component behaviour exists.
    fn get_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<ComponentBehaviourTypeId>;

    /// Returns the count of entity component behaviours.
    fn count(&self) -> usize;
}
