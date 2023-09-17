use thiserror::Error;

use crate::model::DataType;
use crate::BehaviourTypeId;

#[derive(Debug, Error)]
pub enum BehaviourTransitionError {
    #[error("Invalid transition")]
    InvalidTransition,
    #[error("The behaviour is invalid: {0}")]
    BehaviourInvalid(#[from] BehaviourInvalid),
    #[error("Failed to initialize the behaviour: {0}")]
    BehaviourInitializationFailed(#[from] BehaviourInitializationFailed),
    #[error("Connect the behaviour failed: {0}")]
    BehaviourConnectFailed(#[from] BehaviourConnectFailed),
    #[error("Disconnect the behaviour failed: {0}")]
    BehaviourDisconnectFailed(#[from] BehaviourDisconnectFailed),
}

#[derive(Debug, Error)]
pub enum BehaviourCreationError {
    #[error("Creating the behaviour {0} failed because the behaviour is already applied on the reactive instance!")]
    BehaviourAlreadyApplied(BehaviourTypeId),
    /// Creating the behaviour failed because connecting the behaviour failed.
    #[error("Creating the behaviour failed: {0}")]
    BehaviourTransitionError(#[from] BehaviourTransitionError),
}

#[derive(Debug, Error)]
#[error("Connect the behaviour failed")]
pub struct BehaviourConnectFailed {
    // TODO: more detailed reasons
}

#[derive(Debug, Error)]
#[error("Disconnect the behaviour failed")]
pub struct BehaviourDisconnectFailed {
    // TODO: more detailed reasons
}

#[derive(Debug, Error)]
pub enum BehaviourReconnectFailed {
    #[error("Reconnect failed because: Connect the behaviour failed: {0}")]
    BehaviourConnectFailed(#[from] BehaviourConnectFailed),
    #[error("Reconnect failed because: Disconnect the behaviour failed: {0}")]
    BehaviourDisconnectFailed(#[from] BehaviourDisconnectFailed),
}

#[derive(Debug, Error)]
#[error("Failed to initialize the behaviour!")]
pub struct BehaviourInitializationFailed {
    // TODO: more detailed reasons
}

#[derive(Debug, Error)]
#[error("Failed to shut down the behaviour!")]
pub struct BehaviourShutdownFailed {
    // TODO: more detailed reasons
}

#[derive(Debug, Error)]
pub enum BehaviourInvalid {
    #[error("The behaviour is invalid because one or multiple properties are invalid: {0}")]
    BehaviourPropertyInvalid(#[from] BehaviourPropertyInvalid),
}

#[derive(Debug, Error)]
pub enum BehaviourPropertyInvalid {
    #[error("Missing property {0}")]
    PropertyMissing(String),

    #[error("Missing outbound property {0}")]
    OutboundPropertyMissing(String),

    #[error("Missing inbound property {0}")]
    InboundPropertyMissing(String),

    /// The property with the given name has a data type which is not the expected data type.
    #[error("Property {0} has data type {1} but data type {2} was expected!")]
    InvalidDataType(String, DataType, DataType),
}

#[derive(Debug, Error)]
#[error("Failed to apply the behaviour")]
pub struct BehaviourFunctionError;
