use crate::client::error::CommandError;
use crate::client::error::CommandError::NotFound;
use crate::client::types::property_type::args::PropertyTypeDefinitionArgs;
use clap::Args;
use uuid::Uuid;

#[derive(Args, Debug, Clone)]
pub(crate) struct AddPropertyArgs {
    /// The id of the reactive instance.
    pub id: Uuid,

    /// The property to add to the entity instance.
    #[clap(flatten)]
    pub property_type: PropertyTypeDefinitionArgs,
}

impl AddPropertyArgs {
    pub fn id_not_found(&self) -> CommandError {
        NotFound(format!("The instance with the id {} was not found", &self.id))
    }
}
