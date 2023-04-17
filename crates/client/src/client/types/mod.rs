use crate::client::types::components::Components;
use crate::client::InexorRgfClient;
use std::sync::Arc;

pub mod components;

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
}
