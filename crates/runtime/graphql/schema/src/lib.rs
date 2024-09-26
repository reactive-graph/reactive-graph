use async_graphql::EmptySubscription;
use async_graphql::Schema;

pub mod mutation;
pub mod query;

use crate::mutation::RuntimeMutation;
use crate::query::RuntimeQuery;

pub mod instance_address;
pub mod properties;

/// GraphQL Schema for the Reactive Graph Runtime
pub type RuntimeSchema = Schema<RuntimeQuery, RuntimeMutation, EmptySubscription>;
