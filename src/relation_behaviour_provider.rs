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

#[macro_export]
macro_rules! relation_behaviour_provider {
    ($relation_behaviour_provider:expr) => {{
        let relation_behaviour_provider = $relation_behaviour_provider.clone();
        let relation_behaviour_provider: Result<Arc<dyn RelationBehaviourProvider>, _> = <dyn query_interface::Object>::query_arc(relation_behaviour_provider);
        if relation_behaviour_provider.is_err() {
            return Err(RelationBehaviourProviderError::InitializationError);
        }
        Ok(relation_behaviour_provider.ok())
    }};
}
