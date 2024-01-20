use crate::TypeSystemEventTypes;

pub trait TypeSystemEventSubscriber {
    fn subscribe_type_system_event(&self, event_type: TypeSystemEventTypes, handle_id: u128);
    fn unsubscribe_type_system_event(&self, event_type: TypeSystemEventTypes, handle_id: u128);
}
