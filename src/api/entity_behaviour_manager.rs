use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[async_trait]
pub trait EntityBehaviourManager: Send + Sync {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_behaviours_by_id(&self, id: Uuid);

    fn add_provider(&self, behaviour_provider: Arc<dyn EntityBehaviourProvider>);
}
