use std::ops::Deref;

use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use dashmap::DashMap;
use serde_json::Value;

pub struct EventChannels(DashMap<u128, (Sender<Value>, Receiver<Value>)>);

impl EventChannels {
    pub fn new() -> Self {
        Self(DashMap::new())
    }

    pub fn sender(&self, handle_id: &u128) -> Option<Sender<Value>> {
        self.0.get(handle_id).map(|channel| channel.0.clone())
    }

    pub fn receiver(&self, handle_id: &u128) -> Option<Receiver<Value>> {
        self.0.get(handle_id).map(|channel| channel.1.clone())
    }
}

impl Deref for EventChannels {
    type Target = DashMap<u128, (Sender<Value>, Receiver<Value>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
