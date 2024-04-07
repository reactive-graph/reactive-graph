use std::collections::HashSet;

use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_remotes_model::InstanceAddress;
use reactive_graph_remotes_model::InstanceInfo;

use crate::FailedToAddInstance;
use crate::FailedToFetchRemoteInstances;
use crate::FailedToUpdateInstance;

/// Manages the list of remote instances.
#[injectable]
#[async_trait]
pub trait RemotesManager: Send + Sync + Lifecycle {
    /// Returns the remote instances.
    fn get_all(&self) -> Vec<InstanceInfo>;

    /// Returns the instance info with the given address.
    fn get(&self, address: &InstanceAddress) -> Option<InstanceInfo>;

    /// Returns true if the given address is registered.
    fn has(&self, address: &InstanceAddress) -> bool;

    /// Returns a list of the addresses of the remote instances.
    fn get_all_addresses(&self) -> HashSet<InstanceAddress>;

    /// Adds a remote instance.
    async fn add(&self, address: &InstanceAddress) -> Result<InstanceInfo, FailedToAddInstance>;

    /// Removes a remote instance.
    fn remove(&self, address: &InstanceAddress) -> bool;

    /// Removes all remote instances.
    fn remove_all(&self);

    /// Updates a remote instance.
    async fn update(&self, address: &InstanceAddress) -> Result<InstanceInfo, FailedToUpdateInstance>;

    /// Removes all remote instances.
    async fn update_all(&self) -> Vec<InstanceInfo>;

    /// Fetches and adds all remote instances of the given remote instance.
    async fn fetch_and_add_remotes_from_remote(&self, address: &InstanceAddress) -> Result<Vec<InstanceInfo>, FailedToFetchRemoteInstances>;

    /// Fetches and adds all remote instances of all remote instances.
    async fn fetch_and_add_remotes_from_all_remotes(&self) -> Vec<InstanceInfo>;
}
