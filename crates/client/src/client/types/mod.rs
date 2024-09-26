use crate::client::types::components::api::Components;
use crate::client::types::entities::api::EntityTypes;
use crate::client::types::relations::api::RelationTypes;
use crate::client::ReactiveGraphClient;
use std::sync::Arc;

pub mod components;
pub mod entities;
pub mod extensions;
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

    pub fn entity_types(&self) -> EntityTypes {
        EntityTypes::new(self.client.clone())
    }

    pub fn relation_types(&self) -> RelationTypes {
        RelationTypes::new(self.client.clone())
    }
}
