use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;

use reactive_graph_runtime_service_api::ShutdownManager;

use crate::mutation::command::MutationCommands;
use crate::mutation::remotes::MutationRemotes;

pub mod command;
pub mod remotes;

pub struct RuntimeMutation;

/// Mutations for the type system, the instances and the flows.
#[Object(name = "Mutation")]
impl RuntimeMutation {
    async fn commands(&self) -> MutationCommands {
        MutationCommands
    }

    async fn remotes(&self) -> MutationRemotes {
        MutationRemotes
    }

    async fn shutdown(&self, context: &Context<'_>) -> Result<bool> {
        let shutdown_manager = context.data::<Arc<dyn ShutdownManager + Send + Sync>>()?;
        shutdown_manager.do_shutdown();
        Ok(true)
    }
}
