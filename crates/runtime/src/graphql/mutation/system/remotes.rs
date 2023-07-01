use std::sync::Arc;

use async_graphql::*;

use crate::api::RemotesManager;
use crate::graphql::query::GraphQLInstanceInfo;
use crate::model_runtime::InstanceAddress;

#[derive(Default)]
pub struct MutationRemotes;

/// Mutations for managing remote instances.
#[Object]
impl MutationRemotes {
    /// Adds a remote.
    async fn add(&self, context: &Context<'_>, hostname: String, port: u16, secure: Option<bool>) -> Result<GraphQLInstanceInfo> {
        let remotes_manager = context.data::<Arc<dyn RemotesManager>>()?;
        let address = InstanceAddress::new(hostname, port, secure.unwrap_or(false));
        let instance = remotes_manager.add(&address).await?;
        Ok(instance.into())
    }

    /// Removes a remote.
    async fn remove(&self, context: &Context<'_>, hostname: String, port: u16, secure: Option<bool>) -> Result<bool> {
        let remotes_manager = context.data::<Arc<dyn RemotesManager>>()?;
        let address = InstanceAddress::new(hostname, port, secure.unwrap_or(false));
        Ok(remotes_manager.remove(&address))
    }

    /// Removes all remotes.
    async fn remove_all(&self, context: &Context<'_>) -> Result<bool> {
        let remotes_manager = context.data::<Arc<dyn RemotesManager>>()?;
        remotes_manager.remove_all();
        Ok(true)
    }

    /// Updates a remote.
    async fn update(&self, context: &Context<'_>, hostname: String, port: u16, secure: Option<bool>) -> Result<GraphQLInstanceInfo> {
        let remotes_manager = context.data::<Arc<dyn RemotesManager>>()?;
        let address = InstanceAddress::new(hostname, port, secure.unwrap_or(false));
        let instance = remotes_manager.update(&address).await?;
        Ok(instance.into())
    }

    /// Fetches the remotes which are available on the given remote.
    async fn fetch_remotes_from_remote(&self, context: &Context<'_>, hostname: String, port: u16, secure: Option<bool>) -> Result<Vec<GraphQLInstanceInfo>> {
        let remotes_manager = context.data::<Arc<dyn RemotesManager>>()?;
        let address = InstanceAddress::new(hostname, port, secure.unwrap_or(false));
        let added_instances = remotes_manager
            .fetch_and_add_remotes_from_remote(&address)
            .await?
            .into_iter()
            .map(|instance| instance.into())
            .collect();
        Ok(added_instances)
    }
}
