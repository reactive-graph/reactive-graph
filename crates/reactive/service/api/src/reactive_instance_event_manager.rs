use async_trait::async_trait;
use springtime_di::injectable;

use crate::ReactiveInstanceEvent;
use crate::ReactiveInstanceEventTypes;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_model_impl::ReactiveEntity;

pub const REACTIVE_INSTANCE_EVENT_PROPERTY_LABEL: &str = "label";

#[injectable]
#[async_trait]
pub trait ReactiveInstanceEventManager: Send + Sync + Lifecycle {
    /// Emits a system event.
    fn emit_event(&self, event: ReactiveInstanceEvent);

    /// Returns reactive entity instances which can be subscribed to listen for system events.
    fn get_reactive_instance_event_instances(&self) -> Vec<ReactiveEntity>;

    /// Returns the reactive entity instance which can be subscribed to listen for the given system event.
    fn get_reactive_instance_event_instance(&self, event_type: ReactiveInstanceEventTypes) -> Option<ReactiveEntity>;
}
