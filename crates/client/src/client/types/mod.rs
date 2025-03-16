use crate::client::ReactiveGraphClient;
use crate::client::types::components::api::Components;
use crate::client::types::entities::api::EntityTypes;
use crate::client::types::flows::api::FlowTypes;
use crate::client::types::relations::api::RelationTypes;
use std::sync::Arc;

pub mod components;
pub mod entities;
pub mod extensions;
pub mod flows;
pub mod properties;
pub mod relations;

pub struct Types {
    client: Arc<ReactiveGraphClient>,
}

impl Types {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub fn components(&self) -> Components {
        Components::new(self.client.clone())
    }

    pub fn entities(&self) -> EntityTypes {
        EntityTypes::new(self.client.clone())
    }

    pub fn relations(&self) -> RelationTypes {
        RelationTypes::new(self.client.clone())
    }

    pub fn flows(&self) -> FlowTypes {
        FlowTypes::new(self.client.clone())
    }
}
