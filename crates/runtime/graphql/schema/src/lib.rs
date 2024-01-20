use async_graphql::EmptySubscription;
use async_graphql::Schema;

pub mod mutation;
pub mod query;

use crate::mutation::RuntimeMutation;
use crate::query::RuntimeQuery;

pub mod instance_address;
pub mod properties;

/// Inexor GraphQL Interface
pub type RuntimeSchema = Schema<RuntimeQuery, RuntimeMutation, EmptySubscription>;
