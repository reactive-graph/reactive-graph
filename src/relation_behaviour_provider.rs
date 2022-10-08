use std::sync::Arc;

use indradb::EdgeKey;

use crate::model::ReactiveRelationInstance;

#[derive(Debug)]
pub enum RelationBehaviourProviderError {
    InitializationError,
}

pub trait RelationBehaviourProvider: Send + Sync {
    /// Possibly adds new behaviour to the given relation instance
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes behaviour from the given relation instance
    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes behaviour from the given relation instance by edge key
    fn remove_behaviours_by_key(&self, edge_key: &EdgeKey);
}
