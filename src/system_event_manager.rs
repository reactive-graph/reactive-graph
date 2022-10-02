use std::sync::Arc;

use crate::model::ReactiveEntityInstance;
use crate::SystemEventTypes;

pub trait SystemEventManager: Send + Sync {
    /// Returns the reactive entity instance which can be subscribed to listen for the given system event.
    fn get_system_event_instance(&self, event_type: SystemEventTypes) -> Option<Arc<ReactiveEntityInstance>>;
}
