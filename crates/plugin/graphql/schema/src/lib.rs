use async_graphql::EmptySubscription;
use async_graphql::Schema;

use crate::mutation::PluginMutation;
use crate::query::PluginQuery;

pub type PluginSchema = Schema<PluginQuery, PluginMutation, EmptySubscription>;

pub mod mutation;
pub mod query;
