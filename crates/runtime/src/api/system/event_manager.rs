use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::plugins::SystemEvent;
use crate::plugins::SystemEventTypes;
use crate::reactive::ReactiveEntity;

pub const SYSTEM_EVENT_PROPERTY_LABEL: &str = "label";

#[async_trait]
pub trait SystemEventManager: Send + Sync + Lifecycle {
    /// Emits a system event.
    fn emit_event(&self, event: SystemEvent);

    /// Returns reactive entity instances which can be subscribed to listen for system events.
    fn get_system_event_instances(&self) -> Vec<ReactiveEntity>;

    /// Returns the reactive entity instance which can be subscribed to listen for the given system event.
    fn get_system_event_instance(&self, event_type: SystemEventTypes) -> Option<ReactiveEntity>;
}
