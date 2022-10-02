use std::sync::Arc;

use crate::model::ReactiveEntityInstance;
use crate::plugins::SystemEventTypes;

use crate::plugins::SystemEventManager;

pub struct SystemEventManagerImpl {
    system_event_manager: Arc<dyn crate::api::SystemEventManager>,
}

impl SystemEventManagerImpl {
    pub fn new(system_event_manager: Arc<dyn crate::api::SystemEventManager>) -> Self {
        Self { system_event_manager }
    }
}

impl SystemEventManager for SystemEventManagerImpl {
    fn get_system_event_instance(&self, event_type: SystemEventTypes) -> Option<Arc<ReactiveEntityInstance>> {
        self.system_event_manager.get_system_event_instance(event_type)
    }
}
