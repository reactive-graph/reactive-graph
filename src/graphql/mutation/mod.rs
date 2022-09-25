use async_graphql::*;

pub use instances::*;
pub use types::*;

pub mod instances;
pub mod types;

pub struct InexorMutation;

/// Mutations for the type system, the instances and the flows.
#[Object(name = "Mutation")]
impl InexorMutation {
    /// Mutations for types (components, entity types, relation types).
    async fn types(&self) -> MutationTypes {
        MutationTypes::default()
    }

    /// Mutations for instances (entity instances, relation instances).
    async fn instances(&self) -> MutationInstances {
        MutationInstances::default()
    }
}
