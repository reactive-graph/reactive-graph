use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;

use crate::model::ReactiveRelationInstance;

#[async_trait]
pub trait RelationBehaviourManager: Send + Sync {
    /// Adds all behaviours to the given reactive relation instance.
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes all behaviours from the given reactive relation instance.
    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes all behaviours from the reactive relation instance with the given edge key.
    fn remove_behaviours_by_key(&self, edge_key: &EdgeKey);
}
