use std::sync::Arc;

use crate::model::ReactiveEntityInstance;
use uuid::Uuid;

#[derive(Debug)]
pub enum EntityBehaviourProviderError {
    InitializationError,
}

pub trait EntityBehaviourProvider: Send + Sync {
    /// Possibly adds new behaviour to the given entity instance
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Removes behaviour to the given entity instance
    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Removes behaviour to the given entity instance by uuid
    fn remove_behaviours_by_id(&self, id: Uuid);
}

#[macro_export]
macro_rules! entity_behaviour_provider {
    ($entity_behaviour_provider:expr) => {{
        let entity_behaviour_provider = $entity_behaviour_provider.clone();
        let entity_behaviour_provider: Result<Arc<dyn EntityBehaviourProvider>, _> = <dyn query_interface::Object>::query_arc(entity_behaviour_provider);
        if entity_behaviour_provider.is_err() {
            return Err(EntityBehaviourProviderError::InitializationError);
        }
        Ok(entity_behaviour_provider.ok())
    }};
}
