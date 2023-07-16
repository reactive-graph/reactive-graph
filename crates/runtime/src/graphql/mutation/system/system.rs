use std::sync::Arc;

use async_graphql::*;

use crate::api::ShutdownManager;
use crate::graphql::mutation::MutationCommands;
use crate::graphql::mutation::MutationPlugins;
use crate::graphql::mutation::MutationRemotes;

#[derive(Default)]
pub struct MutationSystem;

#[Object]
impl MutationSystem {
    async fn commands(&self) -> MutationCommands {
        MutationCommands
    }

    async fn remotes(&self) -> MutationRemotes {
        MutationRemotes
    }

    async fn plugins(&self) -> MutationPlugins {
        MutationPlugins
    }

    async fn shutdown(&self, context: &Context<'_>) -> Result<bool> {
        let shutdown_manager = context.data::<Arc<dyn ShutdownManager>>()?;
        shutdown_manager.do_shutdown();
        Ok(true)
    }
}
