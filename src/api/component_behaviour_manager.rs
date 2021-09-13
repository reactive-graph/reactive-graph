use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::model::ReactiveEntityInstance;
use crate::plugins::ComponentBehaviourProvider;
use indradb::EdgeKey;
use inexor_rgf_core_model::ReactiveRelationInstance;

#[async_trait]
pub trait ComponentBehaviourManager: Send + Sync {
    /// Adds new behaviours to the given entity instance
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Adds new behaviours to the given relation instance
    fn add_behaviours_to_relation(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes behaviours from the given entity instance
    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Removes behaviours from the given relation instance
    fn remove_behaviours_from_relation(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes behaviours from the given entity instance by uuid
    fn remove_behaviours_by_id(&self, id: Uuid);

    /// Removes behaviours from the given relation instance by edge key
    fn remove_behaviours_by_key(&self, edge_key: EdgeKey);

    fn add_provider(&self, behaviour_provider: Arc<dyn ComponentBehaviourProvider>);
}
