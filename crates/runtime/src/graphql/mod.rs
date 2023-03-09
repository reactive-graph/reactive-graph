use async_graphql::Schema;

pub use mutation::InexorMutation;
pub use query::InexorQuery;
pub use subscription::InexorSubscription;

pub mod directives;
pub mod dynamic;
pub mod mutation;
pub mod query;
pub mod subscription;

/// Inexor GraphQL Interface
pub type InexorSchema = Schema<InexorQuery, InexorMutation, InexorSubscription>;
