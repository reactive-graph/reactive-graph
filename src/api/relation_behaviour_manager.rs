use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;

use crate::model::BehaviourTypeId;
use crate::model::ReactiveRelationInstance;
use crate::reactive::BehaviourTransitionError;

#[async_trait]
pub trait RelationBehaviourManager: Send + Sync {
    /// Adds all behaviours to the given reactive relation instance.
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes all behaviours from the given reactive relation instance.
    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes all behaviours from the reactive relation instance with the given edge key.
    fn remove_behaviours_by_key(&self, edge_key: &EdgeKey);

    /// Connect
    fn connect(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Disconnect
    fn disconnect(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Reconnect
    fn reconnect(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;
}
