use crate::client::instances::entities::api::EntityInstances;
use crate::client::instances::relations::api::RelationInstances;
use crate::InexorRgfClient;
use std::sync::Arc;

pub mod entities;
pub mod relations;
pub mod variables;

pub struct Instances {
    client: Arc<InexorRgfClient>,
}

impl Instances {
    pub fn new(client: Arc<InexorRgfClient>) -> Self {
        Self { client }
    }

    pub fn entity_instances(&self) -> EntityInstances {
        EntityInstances::new(self.client.clone())
    }

    pub fn relation_instances(&self) -> RelationInstances {
        RelationInstances::new(self.client.clone())
    }
}
