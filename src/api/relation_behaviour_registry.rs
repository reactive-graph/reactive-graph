use std::sync::Arc;

use async_trait::async_trait;

use crate::model::BehaviourTypeId;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationBehaviourTypeId;
use crate::model::RelationTypeId;
use crate::reactive::BehaviourFactory;

#[async_trait]
pub trait RelationBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating relation behaviours.
    fn register(&self, relation_behaviour_ty: RelationBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>);

    /// Unregisters a factory for creating relation behaviours.
    fn unregister(&self, relation_behaviour_ty: &RelationBehaviourTypeId);

    /// Returns all relation behaviours.
    fn get_all(&self) -> Vec<RelationBehaviourTypeId>;

    /// Returns the relation behaviour factories for the given entity type.
    fn get(&self, relation_ty: &RelationTypeId) -> Vec<Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>>;

    /// Returns the relation behaviour for the given behaviour type if the relation behaviour exists.
    fn get_factory_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>>;

    /// Returns the relation behaviours for the given entity type.
    fn get_behaviour_types(&self, relation_ty: &RelationTypeId) -> Vec<RelationBehaviourTypeId>;

    /// Returns the relation behaviour for the given behaviour type if the relation behaviour exists.
    fn get_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<RelationBehaviourTypeId>;

    /// Returns the count of relation behaviours.
    fn count(&self) -> usize;
}
