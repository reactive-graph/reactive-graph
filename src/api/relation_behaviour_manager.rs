use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;
use uuid::Uuid;

use crate::model::ReactiveRelationInstance;
use crate::plugins::RelationBehaviourProvider;

#[async_trait]
pub trait RelationBehaviourManager: Send + Sync {
    /// Adds all behaviours to the given reactive relation instance.
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes all behaviours from the given reactive relation instance.
    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes all behaviours from the reactive relation instance with the given edge key.
    fn remove_behaviours_by_key(&self, edge_key: &EdgeKey);

    /// Registers a relation behaviour provider.
    fn add_provider(&self, id: Uuid, behaviour_provider: Arc<dyn RelationBehaviourProvider>);

    /// Unregisters a relation behaviour provider.
    fn remove_provider(&self, id: &Uuid);
}
