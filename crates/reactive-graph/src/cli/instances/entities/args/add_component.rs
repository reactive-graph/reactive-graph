use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NotFound;
use crate::cli::types::components::args::type_id::ComponentTypeIdArgs;
use clap::Args;
use uuid::Uuid;

#[derive(Args, Debug, Clone)]
pub(crate) struct AddComponentArgs {
    /// The id of the reactive instance.
    pub id: Uuid,

    /// The component type to add to the reactive instance.
    #[clap(flatten)]
    pub component_ty: ComponentTypeIdArgs,
}

impl AddComponentArgs {
    pub fn id_not_found(&self) -> CommandError {
        NotFound(format!("The instance with the id {} was not found", &self.id))
    }
}
