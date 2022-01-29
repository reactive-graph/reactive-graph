use crate::model::ReactiveEntityInstance;
use crossbeam::channel::Receiver;
use futures_util::Stream;
use log::debug;
use rand::Rng;
use serde_json::Value;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;
use std::time::Duration;

pub struct EntityPropertyInstanceStream {
    entity_instance: Arc<ReactiveEntityInstance>,
    property_name: String,
    handle_id: u128,
    receiver: Receiver<Value>,
}

impl EntityPropertyInstanceStream {
    pub fn new(entity_instance: Arc<ReactiveEntityInstance>, property_name: String) -> EntityPropertyInstanceStream {
        debug!("Opened subscription entity({})[{}]", entity_instance.id, property_name);
        let mut rng = rand::thread_rng();
        let handle_id = rng.gen::<u128>();
        let property_instance = entity_instance.properties.get(&property_name).unwrap();
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

// #[derive(Default)]
// pub struct Entities;
//
// /// Subscriptions for the reactive property instances.
// #[Subscription]
// impl Entities {
//     async fn property(
//         &self,
//         context: &Context<'_>,
//         #[graphql(desc = "The uuid of the entity instance")] id: Uuid,
//         #[graphql(desc = "The name of the property")] property_name: String,
//     ) -> Result<impl Stream<Item = Value>> {
//         match context.data::<Arc<dyn ReactiveEntityInstanceManager>>() {
//             Ok(entity_instance_manager) => match entity_instance_manager.get(id) {
//                 Some(entity_instance) => {
//                     if !entity_instance.properties.contains_key(&property_name) {
//                         return Err("Error: property by name not found".into());
//                     }
//                     let mut stream = PropertyInstanceStream::new(entity_instance.clone(), property_name.clone());
//
//                     let id = entity_instance.id;
//                     let property_name = property_name.clone();
//                     Ok(async_stream::stream! {
//                         loop {
//                             match stream.next().await {
//                                 Some(v) => {
//                                     futures_timer::Delay::new(Duration::from_millis(10)).await;
//                                     yield json!({
//                                         "id": id,
//                                         "property_name": property_name.clone(),
//                                         "value": v.clone()
//                                     });
//                                 }
//                                 None => {
//                                     futures_timer::Delay::new(Duration::from_millis(100)).await;
//                                 }
//                             };
//                         }
//                     })
//                 }
//                 None => Err("Error: id not found".into()),
//             },
//             Err(_) => Err("Error: REIM".into()),
//         }
//     }
// }
