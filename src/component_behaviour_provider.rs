use std::sync::Arc;

use indradb::EdgeKey;
use inexor_rgf_core_model::{ReactiveEntityInstance, ReactiveRelationInstance};
use uuid::Uuid;

pub trait ComponentBehaviourProvider: Send + Sync {
    /// Possibly adds new behaviour to the given entity instance
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Possibly adds new behaviour to the given relation instance
    fn add_behaviours_to_relation(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes behaviour from the given entity instance
    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Removes behaviour from the given relation instance
    fn remove_behaviours_from_relation(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes behaviour from the given entity instance by uuid
    fn remove_behaviours_by_id(&self, id: Uuid);

    /// Removes behaviour from the given relation instance by edge key
    fn remove_behaviours_by_key(&self, edge_key: EdgeKey);
}
