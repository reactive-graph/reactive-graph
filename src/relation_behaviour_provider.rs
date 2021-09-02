use std::sync::Arc;

use indradb::EdgeKey;
use inexor_rgf_core_model::ReactiveRelationInstance;

pub trait RelationBehaviourProvider: Send + Sync {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey);
}
