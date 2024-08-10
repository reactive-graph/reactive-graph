use crate::client::types::components::api::Components;
use crate::client::types::entity_types::api::EntityTypes;
use crate::client::InexorRgfClient;
use std::sync::Arc;

pub mod components;
pub mod entity_types;
pub mod extensions;

pub struct Types {
    client: Arc<InexorRgfClient>,
}

impl Types {
    pub fn new(client: Arc<InexorRgfClient>) -> Self {
        Self { client }
    }

    pub fn components(&self) -> Components {
        Components::new(self.client.clone())
    }

    pub fn entity_types(&self) -> EntityTypes {
        EntityTypes::new(self.client.clone())
    }
}
