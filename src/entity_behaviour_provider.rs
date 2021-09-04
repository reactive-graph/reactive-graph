use std::sync::Arc;

use inexor_rgf_core_model::ReactiveEntityInstance;
use uuid::Uuid;

pub trait EntityBehaviourProvider: Send + Sync {
    /// Possibly adds new behaviour to the given entity instance
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Removes behaviour to the given entity instance
    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Removes behaviour to the given entity instance by uuid
    fn remove_behaviours_by_id(&self, id: Uuid);
}
