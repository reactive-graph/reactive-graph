use async_graphql::Schema;

pub use mutation::ReactiveGraphMutation;
pub use query::ReactiveGraphQuery;
pub use subscription::ReactiveGraphSubscription;

pub mod directives;
pub mod error;
pub mod mutation;
pub mod query;
pub mod subscription;

/// GraphQL Schema for the Reactive Graph
pub type ReactiveGraphSchema = Schema<ReactiveGraphQuery, ReactiveGraphMutation, ReactiveGraphSubscription>;

#[cfg(test)]
mod tests;
