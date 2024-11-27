pub mod instances;
pub mod types;

use crate::query::json_schema::instances::JsonSchemaInstanceSystem;
use crate::query::json_schema::types::JsonSchemaTypeSystem;
use async_graphql::Object;

#[derive(Default)]
pub struct JsonSchema;

/// Get the JSON schema of the type system and instance system.
#[Object]
impl JsonSchema {
    async fn types(&self) -> JsonSchemaTypeSystem {
        JsonSchemaTypeSystem
    }
    async fn instances(&self) -> JsonSchemaInstanceSystem {
        JsonSchemaInstanceSystem
    }
}
