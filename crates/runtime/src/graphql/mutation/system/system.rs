use crate::graphql::mutation::MutationCommands;
use crate::graphql::mutation::MutationPlugins;
use crate::graphql::mutation::MutationRemotes;
use async_graphql::*;

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
}
