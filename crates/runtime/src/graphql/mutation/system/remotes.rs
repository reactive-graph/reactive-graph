use std::sync::Arc;

use async_graphql::*;

use crate::api::RemotesManager;
use crate::graphql::mutation::InstanceAddressDefinition;
use crate::graphql::query::GraphQLInstanceInfo;

#[derive(Default)]
pub struct MutationRemotes;

/// Mutations for managing remote instances.
#[Object]
impl MutationRemotes {
    /// Adds a remote.
    async fn add(&self, context: &Context<'_>, address: InstanceAddressDefinition, fetch_remotes_from_remote: Option<bool>) -> Result<GraphQLInstanceInfo> {
        let remotes_manager = context.data::<Arc<dyn RemotesManager>>()?;
        let instance = remotes_manager.add(&address.into()).await?;
        if fetch_remotes_from_remote.unwrap_or(false) {
            let _ = remotes_manager.fetch_and_add_remotes_from_remote(&instance.address).await;
        }
        Ok(instance.into())
    }

    /// Removes a remote.
    async fn remove(&self, context: &Context<'_>, address: InstanceAddressDefinition) -> Result<bool> {
        let remotes_manager = context.data::<Arc<dyn RemotesManager>>()?;
        Ok(remotes_manager.remove(&address.into()))
    }

    /// Removes all remotes.
    async fn remove_all(&self, context: &Context<'_>) -> Result<bool> {
        let remotes_manager = context.data::<Arc<dyn RemotesManager>>()?;
        remotes_manager.remove_all();
        Ok(true)
    }

    /// Updates a remote.
    async fn update(&self, context: &Context<'_>, address: InstanceAddressDefinition) -> Result<GraphQLInstanceInfo> {
        let remotes_manager = context.data::<Arc<dyn RemotesManager>>()?;
        let instance = remotes_manager.update(&address.into()).await?;
        Ok(instance.into())
    }

    /// Updates all remotes.
    async fn update_all(&self, context: &Context<'_>) -> Result<Vec<GraphQLInstanceInfo>> {
        let remotes_manager = context.data::<Arc<dyn RemotesManager>>()?;
        let updated_remotes = remotes_manager.update_all().await.into_iter().map(|instance| instance.into()).collect();
        Ok(updated_remotes)
    }

    /// Fetches the remotes which are available on the given remote.
    async fn fetch_remotes_from_remote(&self, context: &Context<'_>, address: InstanceAddressDefinition) -> Result<Vec<GraphQLInstanceInfo>> {
        let remotes_manager = context.data::<Arc<dyn RemotesManager>>()?;
        let added_instances = remotes_manager
            .fetch_and_add_remotes_from_remote(&address.into())
            .await?
            .into_iter()
            .map(|instance| instance.into())
            .collect();
        Ok(added_instances)
    }

    async fn fetch_remotes_from_all_remotes(&self, context: &Context<'_>) -> Result<Vec<GraphQLInstanceInfo>> {
        let remotes_manager = context.data::<Arc<dyn RemotesManager>>()?;
        let added_instances = remotes_manager
            .fetch_and_add_remotes_from_all_remotes()
            .await
            .into_iter()
            .map(|instance| instance.into())
            .collect();
        Ok(added_instances)
    }
}
