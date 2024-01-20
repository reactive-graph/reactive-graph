use async_graphql::*;
use uuid::Uuid;

pub use behaviours::*;
pub use instances::*;
pub use types::*;

pub mod behaviours;
pub mod instances;
// pub mod system;
pub mod types;

pub struct InexorQuery;

/// Search queries for the type system, the instances and the flows.
#[Object(name = "Query")]
impl InexorQuery {
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

    // async fn system(&self) -> System {
    //     System
    // }

    async fn random_uuid(&self, _context: &Context<'_>) -> String {
        Uuid::new_v4().to_string()
    }
}
