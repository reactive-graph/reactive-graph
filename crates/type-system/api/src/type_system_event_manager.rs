use async_trait::async_trait;
use springtime_di::injectable;

use crate::TypeSystemEvent;
use crate::TypeSystemEventTypes;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveEntity;

pub const TYPE_SYSTEM_EVENT_PROPERTY_LABEL: &str = "label";

#[injectable]
#[async_trait]
pub trait TypeSystemEventManager: Send + Sync + Lifecycle {
    /// Emits a system event.
    fn emit_event(&self, event: TypeSystemEvent);

    /// Returns reactive entity instances which can be subscribed to listen for system events.
    fn get_type_system_event_instances(&self) -> Vec<ReactiveEntity>;

    /// Returns the reactive entity instance which can be subscribed to listen for the given system event.
    fn get_type_system_event_instance(&self, event_type: TypeSystemEventTypes) -> Option<ReactiveEntity>;
}
