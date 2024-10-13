use crate::client::error::CommandError;
use crate::client::instances::relations::args::id::RelationInstanceIdArgs;
use crate::client::types::property_type::args::PropertyTypeDefinitionArgs;
use clap::Args;

#[derive(Args, Debug, Clone)]
pub(crate) struct AddPropertyArgs {
    /// The id of the relation instance.
    #[clap(flatten)]
    pub id: RelationInstanceIdArgs,

    /// The property to add to the relation instance.
    #[clap(flatten)]
    pub property_type: PropertyTypeDefinitionArgs,
}

impl AddPropertyArgs {
    pub fn id_not_found(&self) -> CommandError {
        self.id.not_found()
    }
}
