use async_graphql::{EmptySubscription, Schema};

pub use mutation::InexorMutation;
pub use query::InexorQuery;

pub mod mutation;
pub mod query;

/// Inexor GraphQL Interface
pub type InexorSchema = Schema<InexorQuery, InexorMutation, EmptySubscription>;
