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

    /// Creates and adds the given behaviour to the given reactive entity instance.
    fn add_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId);

    /// Removes the given behaviour from the given reactive relation instance.
    fn remove_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId);

    /// Removes all behaviours from the given reactive relation instance.
    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes all behaviours from the reactive relation instance with the given edge key.
    fn remove_behaviours_by_key(&self, edge_key: &EdgeKey);

    /// Removes all behaviours of the given behaviour type.
    fn remove_behaviours_by_behaviour(&self, behaviour_ty: &BehaviourTypeId);

    /// Returns true, if the relation instance has the given behaviour.
    fn has(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) -> bool;

    /// Returns the behaviours of the given relation instance.
    fn get_all(&self, relation_instance: Arc<ReactiveRelationInstance>) -> Vec<BehaviourTypeId>;

    /// Returns the relation instances with the given behaviour.
    fn get_instances_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<Arc<ReactiveRelationInstance>>;

    /// Connect
    fn connect(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Disconnect
    fn disconnect(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Reconnect
    fn reconnect(&self, relation_instance: Arc<ReactiveRelationInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;
}
