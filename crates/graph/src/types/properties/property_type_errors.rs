use thiserror::Error;

#[derive(Debug, Error)]
pub enum AddPropertyError {
    #[error("The property with name {0} already exists")]
    PropertyAlreadyExist(String),
}

#[derive(Debug, Error)]
pub enum UpdatePropertyError {
    #[error("The property with name {0} does not exist")]
    PropertyDoesNotExist(String),
}

#[derive(Debug, Error)]
pub enum RemovePropertyError {
    #[error("The property with name {0} does not exist")]
    PropertyDoesNotExist(String),
}
