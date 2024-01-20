use std::pin::Pin;
use std::task::Poll;
use std::time::Duration;

use crossbeam::channel::Receiver;
use futures_util::Stream;
use log::debug;
use rand::Rng;
use serde_json::Value;

use inexor_rgf_reactive_model_impl::ReactiveEntity;

pub struct EntityPropertyInstanceStream {
    entity_instance: ReactiveEntity,
    property_name: String,
    handle_id: u128,
    receiver: Receiver<Value>,
}

impl EntityPropertyInstanceStream {
    pub fn new(entity_instance: ReactiveEntity, property_name: String) -> EntityPropertyInstanceStream {
        debug!("Opened subscription entity({})[{}]", entity_instance.id, property_name);
        let mut rng = rand::thread_rng();
        let handle_id = rng.gen::<u128>();
        let entity_instance2 = entity_instance.clone();
        let property_instance = entity_instance2.properties.get(&property_name).unwrap();
        let (sender, receiver) = crossbeam::channel::unbounded();
        property_instance.stream.read().unwrap().observe_with_handle(
            move |value: &Value| {
                let _ = sender.send(value.clone());
            },
            handle_id,
        );
        EntityPropertyInstanceStream {
            entity_instance,
            property_name,
            handle_id,
            receiver,
        }
    }
}

impl Stream for EntityPropertyInstanceStream {
    type Item = Value;

    fn poll_next(self: Pin<&mut Self>, _context: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        match self.receiver.try_recv() {
            Ok(value) => {
                std::thread::sleep(Duration::from_millis(10));
                Poll::Ready(Some(value))
            }
            Err(_) => {
                std::thread::sleep(Duration::from_millis(100));
                Poll::Ready(None)
            }
        }
    }
}

impl Drop for EntityPropertyInstanceStream {
    fn drop(&mut self) {
        debug!("Closing subscription entity({})[{}]", self.entity_instance.id, self.property_name.clone());
        let property_instance = self.entity_instance.properties.get(self.property_name.as_str()).unwrap();
        property_instance.stream.read().unwrap().remove(self.handle_id);
    }
}
