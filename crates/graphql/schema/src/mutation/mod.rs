use async_graphql::Object;

pub use instances::*;
// pub use system::*;
pub use types::*;

pub mod instances;
// pub mod system;
pub mod types;

pub struct ReactiveGraphMutation;

/// Mutations for the type system, the instances and the flows.
#[Object(name = "Mutation")]
impl ReactiveGraphMutation {
    /// Mutations for types (components, entity types, relation types).
    async fn types(&self) -> MutationTypes {
        MutationTypes
    }

    /// Mutations for instances (entity instances, relation instances).
    async fn instances(&self) -> MutationInstances {
        MutationInstances
    }
}
