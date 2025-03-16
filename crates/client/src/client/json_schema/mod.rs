use crate::ReactiveGraphClient;
use crate::client::json_schema::instances::JsonSchemaInstanceSystem;
use crate::client::json_schema::types::JsonSchemaTypeSystem;
use std::sync::Arc;

pub mod instances;
pub mod types;

pub struct JsonSchema {
    client: Arc<ReactiveGraphClient>,
}

impl JsonSchema {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub fn instances(&self) -> JsonSchemaInstanceSystem {
        JsonSchemaInstanceSystem::new(self.client.clone())
    }

    pub fn types(&self) -> JsonSchemaTypeSystem {
        JsonSchemaTypeSystem::new(self.client.clone())
    }
}
