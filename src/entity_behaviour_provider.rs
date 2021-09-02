use std::sync::Arc;

use inexor_rgf_core_model::ReactiveEntityInstance;
use uuid::Uuid;

pub trait EntityBehaviourProvider: Send + Sync {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_behaviours_by_id(&self, id: Uuid);
}
