use std::pin::Pin;
use std::task::Poll;
use std::time::Duration;

use crossbeam::channel::Receiver;
use futures_util::Stream;
use log::debug;
use rand::Rng;
use serde_json::Value;

use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_reactive_model_impl::ReactiveRelation;

pub struct RelationPropertyInstanceStream {
    relation_instance: ReactiveRelation,
    property_name: String,
    handle_id: u128,
    receiver: Receiver<Value>,
}

impl RelationPropertyInstanceStream {
    pub fn new(relation_instance: ReactiveRelation, property_name: String) -> RelationPropertyInstanceStream {
        debug!(
            "Opened subscription relation({}__{}__{})[{}]",
            relation_instance.inbound.id,
            relation_instance.type_definition(),
            relation_instance.outbound.id,
            property_name
        );
        let mut rng = rand::rng();
        let handle_id = rng.random::<u128>();
        let relation_instance2 = relation_instance.clone();
        let property_instance = relation_instance2.properties.get(&property_name).unwrap();
        let (sender, receiver) = crossbeam::channel::unbounded();
        property_instance.stream.read().unwrap().observe_with_handle(
            move |value: &Value| {
                let _ = sender.send(value.clone());
            },
            handle_id,
        );
        RelationPropertyInstanceStream {
            relation_instance,
            property_name,
            handle_id,
            receiver,
        }
    }
}

impl Stream for RelationPropertyInstanceStream {
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

impl Drop for RelationPropertyInstanceStream {
    fn drop(&mut self) {
        debug!(
            "Closing subscription relation({}__{}__{})[{}]",
            self.relation_instance.inbound.id,
            self.relation_instance.type_definition(),
            self.relation_instance.outbound.id,
            self.property_name.clone()
        );
        let property_instance = self.relation_instance.properties.get(self.property_name.as_str()).unwrap();
        property_instance.stream.read().unwrap().remove(self.handle_id);
    }
}
