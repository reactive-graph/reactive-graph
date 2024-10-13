use crate::client::error::CommandError;
use crate::client::error::CommandError::NotFound;
use crate::client::instances::properties::args::PropertyInstanceArgs;
use crate::client::instances::relations::args::id::RelationInstanceIdArgs;
use clap::Args;
use reactive_graph_graph::RelationInstanceId;

/// CLI argument for searching relation instances.
#[derive(Args, Debug, Clone)]
pub(crate) struct SetPropertyArgs {
    /// The id of the relation instance.
    #[clap(flatten)]
    pub id: RelationInstanceIdArgs,

    #[clap(flatten)]
    pub property_instance: PropertyInstanceArgs,
}

impl SetPropertyArgs {
    pub fn not_found(&self) -> CommandError {
        self.id.not_found()
    }

    pub fn property_not_found(&self) -> CommandError {
        let id: RelationInstanceId = self.into();
        NotFound(format!("The relation instance with the id {} has no property {}", id, &self.property_instance.property_name))
    }
}

impl From<&SetPropertyArgs> for RelationInstanceId {
    fn from(args: &SetPropertyArgs) -> Self {
        RelationInstanceId::from(&args.id)
    }
}
