use crate::graphql::mutation::MutationPlugins;
use async_graphql::*;

#[derive(Default)]
pub struct MutationSystem;

#[Object]
impl MutationSystem {
    async fn plugins(&self) -> MutationPlugins {
        MutationPlugins::default()
    }
}
