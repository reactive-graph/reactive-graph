use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NotFound;
use clap::Args;
use uuid::Uuid;

/// CLI argument which identifies a property of an entity instance.
#[derive(Args, Debug, Clone)]
pub(crate) struct IdAndPropertyArgs {
    /// The id of the reactive instance.
    pub id: Uuid,

    /// The name of the property.
    pub property_name: String,
}

impl IdAndPropertyArgs {
    pub fn id_not_found(&self) -> CommandError {
        NotFound(format!("The instance with the id {} was not found", &self.id))
    }

    pub fn property_not_found(&self) -> CommandError {
        NotFound(format!("The instance with the id {} has no property {}", &self.id, &self.property_name))
    }
}
