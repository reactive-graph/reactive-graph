use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;

use crate::model::ReactiveRelationInstance;
use crate::plugins::RelationBehaviourProvider;

#[async_trait]
pub trait RelationBehaviourManager: Send + Sync {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey);

    fn add_provider(&self, behaviour_provider: Arc<dyn RelationBehaviourProvider>);
}
