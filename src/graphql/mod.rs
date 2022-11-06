use async_graphql::Schema;

pub use mutation::InexorMutation;
pub use query::InexorQuery;
pub use subscription::InexorSubscription;

pub mod directives;
pub mod mutation;
pub mod query;
pub mod subscription;

// pub mod dynamic_graph;

/// Inexor GraphQL Interface
pub type InexorSchema = Schema<InexorQuery, InexorMutation, InexorSubscription>;
