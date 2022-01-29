use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

/// The EntityBehaviourManager is a registry for EntityBehaviourProviders and provides
/// functionality to add and remove known behaviours onto/from a reactive entity instance.
#[async_trait]
pub trait EntityBehaviourManager: Send + Sync {
    /// Adds all behaviours to the given reactive entity instance.
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Removes all behaviours from the given reactive entity instance.
    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Removes all behaviours from the reactive entity instance with the given id.
    fn remove_behaviours_by_id(&self, id: Uuid);

    /// Registers a entity behaviour provider.
    fn add_provider(&self, behaviour_provider: Arc<dyn EntityBehaviourProvider>);
}
