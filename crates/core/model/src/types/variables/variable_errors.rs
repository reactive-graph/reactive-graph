use thiserror::Error;
use crate::AddPropertyError;
use crate::RemovePropertyError;
use crate::UpdatePropertyError;

#[derive(Debug, Error)]
pub enum AddVariableError {
    #[error("The variable with name {0} already exists")]
    VariableAlreadyExist(String),
}

impl From<AddPropertyError> for AddVariableError {
    fn from(e: AddPropertyError) -> Self {
        match e {
            AddPropertyError::PropertyAlreadyExist(variable_name) => AddVariableError::VariableAlreadyExist(variable_name)
        }

    }
}

#[derive(Debug, Error)]
pub enum UpdateVariableError {
    #[error("The variable with name {0} does not exist")]
    VariableDoesNotExist(String),
}

impl From<UpdatePropertyError> for UpdateVariableError {
    fn from(e: UpdatePropertyError) -> Self {
        match e {
            UpdatePropertyError::PropertyDoesNotExist(variable_name) => UpdateVariableError::VariableDoesNotExist(variable_name)
        }

    }
}

#[derive(Debug, Error)]
pub enum RemoveVariableError {
    #[error("The variable with name {0} does not exist")]
    VariableDoesNotExist(String),
}

impl From<RemovePropertyError> for RemoveVariableError {
    fn from(e: RemovePropertyError) -> Self {
        match e {
            RemovePropertyError::PropertyDoesNotExist(variable_name) => RemoveVariableError::VariableDoesNotExist(variable_name)
        }

    }
}
