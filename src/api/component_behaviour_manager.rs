use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::model::Component;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveRelationInstance;
use crate::plugins::ComponentBehaviourProvider;
use indradb::EdgeKey;

#[async_trait]
pub trait ComponentBehaviourManager: Send + Sync {
    /// Adds new behaviours to the given entity instance.
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Possibly adds new behaviour to the given entity instance's component
    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: Component);

    /// Adds new behaviours to the given relation instance.
    fn add_behaviours_to_relation(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Possibly adds new behaviour to the given relation instance's component
    fn add_behaviours_to_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: Component);

    /// Removes behaviours from the given entity instance.
    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Removes behaviour from the given entity instance's component
    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: Component);

    /// Removes behaviours from the given relation instance.
    fn remove_behaviours_from_relation(&self, relation_instance: Arc<ReactiveRelationInstance>);

    /// Removes behaviour from the given relation instance's component
    fn remove_behaviours_from_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: Component);

    /// Removes behaviours from the given entity instance by uuid.
    fn remove_behaviours_by_id(&self, id: Uuid);

    /// Removes behaviours from the given relation instance by edge key.
    fn remove_behaviours_by_key(&self, edge_key: EdgeKey);

    /// Registers a component behaviour provider.
    fn add_provider(&self, behaviour_provider: Arc<dyn ComponentBehaviourProvider>);
}
