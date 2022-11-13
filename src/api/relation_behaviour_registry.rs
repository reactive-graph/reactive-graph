use std::sync::Arc;

use async_trait::async_trait;

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

    /// Returns the relation behaviour factories for the given entity type.
    fn get(&self, relation_ty: &RelationTypeId) -> Vec<Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>>;
}
