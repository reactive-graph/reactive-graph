use crate::ReactiveGraphClient;
use crate::client::instances::entities::api::EntityInstances;
use crate::client::instances::flows::api::FlowInstances;
use crate::client::instances::relations::api::RelationInstances;
use std::sync::Arc;

pub mod entities;
pub mod flows;
pub mod properties;
pub mod relations;
pub mod variables;

pub struct Instances {
    client: Arc<ReactiveGraphClient>,
}

impl Instances {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub fn entities(&self) -> EntityInstances {
        EntityInstances::new(self.client.clone())
    }

    pub fn relations(&self) -> RelationInstances {
        RelationInstances::new(self.client.clone())
    }

    pub fn flows(&self) -> FlowInstances {
        FlowInstances::new(self.client.clone())
    }
}
