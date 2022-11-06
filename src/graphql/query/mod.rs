use async_graphql::*;
use uuid::Uuid;

pub use instances::*;
pub use types::*;

use crate::graphql::query::Instances;
use crate::graphql::query::Types;

pub mod instances;
pub mod types;

pub struct InexorQuery;

/// Search queries for the type system, the instances and the flows.
#[Object(name = "Query")]
impl InexorQuery {
    /// Search for types (components, entity types, relation types).
    async fn types(&self) -> Types {
        Types::default()
    }

    /// Search for instances (entity instances, relation instances).
    async fn instances(&self) -> Instances {
        Instances::default()
    }

    async fn random_uuid(&self, _context: &Context<'_>) -> String {
        Uuid::new_v4().to_string()
    }
}
