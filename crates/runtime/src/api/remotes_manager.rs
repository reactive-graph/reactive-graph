use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use gql_client::GraphQLError;

use crate::api::Lifecycle;
use crate::config::InstanceAddress;
use crate::model_runtime::InstanceInfo;

#[derive(Debug)]
pub enum FailedToAddInstance {
    InstanceAddressAlreadyExists,
    FailedToFetchInstanceInfo(FailedToFetchInstanceInfo),
    InstanceNotAdded,
}

impl fmt::Display for FailedToAddInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FailedToAddInstance::InstanceAddressAlreadyExists => {
                write!(f, "The instance address can't be added because the instance address is already registered")
            }
            FailedToAddInstance::FailedToFetchInstanceInfo(e) => write!(f, "Failed to fetch instance info: {}", e),
            FailedToAddInstance::InstanceNotAdded => write!(f, "The instance info was not added"),
        }
    }
}

impl Error for FailedToAddInstance {}

#[derive(Debug)]
pub enum FailedToUpdateInstance {
    InstanceAddressDoesNotExist,
    FailedToFetchInstanceInfo(FailedToFetchInstanceInfo),
    InstanceNotUpdated,
}

impl fmt::Display for FailedToUpdateInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FailedToUpdateInstance::InstanceAddressDoesNotExist => write!(f, "The instance address can't be updated because it doesn't exist."),
            FailedToUpdateInstance::FailedToFetchInstanceInfo(e) => write!(f, "Failed to fetch instance info: {}", e),
            FailedToUpdateInstance::InstanceNotUpdated => write!(f, "The instance info was not updated"),
        }
    }
}

impl Error for FailedToUpdateInstance {}

#[derive(Debug)]
pub enum FailedToFetchInstanceInfo {
    RequestError(GraphQLError),
    InvalidResponseData,
}

impl fmt::Display for FailedToFetchInstanceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FailedToFetchInstanceInfo::RequestError(e) => write!(f, "The instance info request errored: {}", e),
            FailedToFetchInstanceInfo::InvalidResponseData => write!(f, "The instance info response data is invalid"),
        }
    }
}

impl Error for FailedToFetchInstanceInfo {}

#[derive(Debug)]
pub enum FailedToFetchRemoteInstances {
    RequestError(GraphQLError),
    InvalidResponseData,
}

impl fmt::Display for FailedToFetchRemoteInstances {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FailedToFetchRemoteInstances::RequestError(e) => write!(f, "The remote instances request errored: {}", e),
            FailedToFetchRemoteInstances::InvalidResponseData => write!(f, "The remote instances response data is invalid"),
        }
    }
}

impl Error for FailedToFetchRemoteInstances {}

/// Manages the list of remote instances.
#[async_trait]
pub trait RemotesManager: Send + Sync + Lifecycle {
    /// Returns the remote instances.
    fn get_all(&self) -> Vec<InstanceInfo>;

    /// Returns the instance info with the given address.
    fn get(&self, address: &InstanceAddress) -> Option<InstanceInfo>;

    /// Returns true if the given address is registered.
    fn has(&self, address: &InstanceAddress) -> bool;

    /// Returns a list of the addresses of the remote instances.
    fn get_all_addresses(&self) -> Vec<InstanceAddress>;

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
