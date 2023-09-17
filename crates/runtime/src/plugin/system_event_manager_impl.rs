use std::sync::Arc;

use crate::plugins::SystemEventManager;
use crate::plugins::SystemEventTypes;
use crate::reactive::ReactiveEntity;

pub struct SystemEventManagerImpl {
    system_event_manager: Arc<dyn crate::api::SystemEventManager>,
}

impl SystemEventManagerImpl {
    pub fn new(system_event_manager: Arc<dyn crate::api::SystemEventManager>) -> Self {
        Self { system_event_manager }
    }
}

impl SystemEventManager for SystemEventManagerImpl {
    fn get_system_event_instance(&self, event_type: SystemEventTypes) -> Option<ReactiveEntity> {
        self.system_event_manager.get_system_event_instance(event_type)
    }
}
