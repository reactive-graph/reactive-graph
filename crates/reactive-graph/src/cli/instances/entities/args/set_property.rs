use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NotFound;
use clap::Args;
use serde_json::Value;
use uuid::Uuid;

/// CLI argument which identifies a reactive instance by its id.
#[derive(Args, Debug, Clone)]
pub(crate) struct SetPropertyArgs {
    /// The id of the reactive instance.
    pub id: Uuid,

    /// The name of the property.
    pub name: String,

    /// The new value of the property.
    pub value: Value,
}

impl SetPropertyArgs {
    pub fn id_not_found(&self) -> CommandError {
        NotFound(format!("The instance with the id {} was not found", &self.id))
    }

    pub fn property_not_found(&self) -> CommandError {
        NotFound(format!("The instance with the id {} has no property {}", &self.id, &self.name))
    }
}
