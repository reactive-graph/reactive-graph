use std::sync::Arc;

use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_type_system_api::TypeSystemEventTypes;

pub struct TypeSystemEventManagerDelegate {
    type_system_event_manager: Arc<dyn reactive_graph_type_system_api::TypeSystemEventManager + Send + Sync>,
}

impl TypeSystemEventManagerDelegate {
    pub fn new(type_system_event_manager: Arc<dyn reactive_graph_type_system_api::TypeSystemEventManager + Send + Sync>) -> Self {
        Self { type_system_event_manager }
    }
}

impl reactive_graph_plugin_api::TypeSystemEventManager for TypeSystemEventManagerDelegate {
    fn get_type_system_event_instance(&self, event_type: TypeSystemEventTypes) -> Option<ReactiveEntity> {
        self.type_system_event_manager.get_type_system_event_instance(event_type)
    }
}
