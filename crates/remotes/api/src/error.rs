use gql_client::GraphQLError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FailedToAddInstance {
    #[error("The instance address can't be added because the instance address is already registered")]
    InstanceAddressAlreadyExists,
    #[error("Failed to fetch instance info: {0}")]
    FailedToFetchInstanceInfo(FailedToFetchInstanceInfo),
    #[error("The instance info was not added")]
    InstanceNotAdded,
}

#[derive(Debug, Error)]
pub enum FailedToUpdateInstance {
    #[error("The instance address can't be updated because it doesn't exist!")]
    InstanceAddressDoesNotExist,
    #[error("Failed to fetch instance info: {0}")]
    FailedToFetchInstanceInfo(FailedToFetchInstanceInfo),
    #[error("The instance info wasn't updated!")]
    InstanceNotUpdated,
}

#[derive(Debug, Error)]
pub enum FailedToFetchInstanceInfo {
    #[error("The instance info request errored: {0}")]
    RequestError(GraphQLError),
    #[error("The instance info response data is invalid")]
    InvalidResponseData,
}

#[derive(Debug, Error)]
pub enum FailedToFetchRemoteInstances {
    #[error("The remote instances request errored: {0}")]
    RequestError(GraphQLError),
    #[error("The remote instances response data is invalid")]
    InvalidResponseData,
}
