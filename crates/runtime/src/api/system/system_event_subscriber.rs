use crate::plugins::SystemEventTypes;

pub trait SystemEventSubscriber {
    fn subscribe_system_event(&self, system_event_type: SystemEventTypes, handle_id: u128);
    fn unsubscribe_system_event(&self, system_event_type: SystemEventTypes, handle_id: u128);
}
