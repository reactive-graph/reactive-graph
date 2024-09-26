use crate::client::instances::entities::api::EntityInstances;
use crate::client::instances::relations::api::RelationInstances;
use crate::ReactiveGraphClient;
use std::sync::Arc;

pub mod entities;
pub mod relations;
pub mod variables;

pub struct Instances {
    client: Arc<ReactiveGraphClient>,
}

impl Instances {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub fn entity_instances(&self) -> EntityInstances {
        EntityInstances::new(self.client.clone())
    }

    pub fn relation_instances(&self) -> RelationInstances {
        RelationInstances::new(self.client.clone())
    }
}
