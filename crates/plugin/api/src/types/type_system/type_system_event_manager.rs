use springtime_di::injectable;

use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_type_system_api::TypeSystemEventTypes;

#[injectable]
pub trait TypeSystemEventManager: Send + Sync {
    /// Returns the reactive entity instance which can be subscribed to listen for the given system event.
    fn get_type_system_event_instance(&self, event_type: TypeSystemEventTypes) -> Option<ReactiveEntity>;
}
