use crate::ReactiveInstanceEventTypes;

pub trait ReactiveInstanceEventSubscriber {
    fn subscribe_reactive_instance_event(&self, reactive_instance_event_type: ReactiveInstanceEventTypes, handle_id: u128);
    fn unsubscribe_reactive_instance_event(&self, reactive_instance_event_type: ReactiveInstanceEventTypes, handle_id: u128);
}
