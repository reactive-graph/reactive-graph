use std::sync::Arc;

use async_trait::async_trait;
use inexor_rgf_core_model::RelationInstanceId;

use crate::behaviour::BehaviourFactory;
use crate::model::ComponentTypeId;
use crate::reactive::BehaviourTypeId;
use crate::reactive::ComponentBehaviourTypeId;
use crate::reactive::ReactiveRelation;

#[async_trait]
pub trait RelationComponentBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating relation component behaviours.
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>);

    /// Unregisters a factory for creating relation component behaviours.
    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId);

    /// Returns all relation component behaviours.
    fn get_all(&self) -> Vec<ComponentBehaviourTypeId>;

    /// Returns the relation behaviour factories for the given component type.
    fn get(&self, component_ty: &ComponentTypeId) -> Vec<Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>>;

    /// Returns the component behaviours for the given component type.
    fn get_behaviour_types(&self, component_ty: &ComponentTypeId) -> Vec<ComponentBehaviourTypeId>;

    /// Returns the entity behaviour for the given behaviour type if the relation component behaviour exists.
    fn get_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<ComponentBehaviourTypeId>;

    /// Returns the count of relation component behaviours.
    fn count(&self) -> usize;
}
