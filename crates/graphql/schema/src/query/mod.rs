use async_graphql::Context;
use async_graphql::Object;
use uuid::Uuid;

pub use behaviours::*;
pub use instances::*;
pub use json_schema::*;
pub use types::*;

pub mod behaviours;
pub mod instances;
pub mod json_schema;
pub mod types;

pub struct ReactiveGraphQuery;

/// Search queries for the type system, the instances and the flows.
#[Object(name = "Query")]
impl ReactiveGraphQuery {
    /// Search for types (components, entity types, relation types).
    async fn types(&self) -> Types {
        Types
    }

    /// Search for instances (entity instances, relation instances).
    async fn instances(&self) -> Instances {
        Instances
    }

    /// Search for behaviours (entity behaviours, entity component behaviours, relation behaviours,
    /// relation component behaviours).
    async fn behaviours(&self) -> Behaviours {
        Behaviours
    }

    /// JSON Schema definitions.
    async fn json_schema(&self) -> JsonSchema {
        JsonSchema
    }

    async fn random_uuid(&self, _context: &Context<'_>) -> String {
        Uuid::new_v4().to_string()
    }
}
