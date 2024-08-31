use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NotFound;
use crate::cli::types::property_type::args::PropertyTypeDefinitionArgs;
use clap::Args;
use uuid::Uuid;

/// CLI argument which identifies a reactive instance by its id.
#[derive(Args, Debug, Clone)]
pub(crate) struct AddPropertyArgs {
    /// The id of the reactive instance.
    pub id: Uuid,

    #[clap(flatten)]
    pub property_type: PropertyTypeDefinitionArgs,
}

impl AddPropertyArgs {
    pub fn id_not_found(&self) -> CommandError {
        NotFound(format!("The instance with the id {} was not found", &self.id))
    }
}
