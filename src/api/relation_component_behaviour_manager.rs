use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;

use crate::model::Component;
use crate::model::ReactiveRelationInstance;

#[async_trait]
pub trait RelationComponentBehaviourManager: Send + Sync {
    /// Adds new behaviours to the given relation instance.
    fn add_behaviours_to_relation(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Possibly adds new behaviour to the given relation instance's component
    fn add_behaviours_to_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: Component);

    /// Removes behaviours from the given relation instance.
    fn remove_behaviours_from_relation(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes behaviour from the given relation instance's component
    fn remove_behaviours_from_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: Component);

    /// Removes behaviours from the given relation instance by edge key.
    fn remove_behaviours_by_key(&self, edge_key: &EdgeKey);
}
