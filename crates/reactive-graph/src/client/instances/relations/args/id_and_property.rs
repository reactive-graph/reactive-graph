use crate::client::error::CommandError;
use crate::client::error::CommandError::NotFound;
use crate::client::instances::relations::args::id::RelationInstanceIdArgs;
use clap::Args;
use reactive_graph_graph::RelationInstanceId;

/// CLI argument which identifies a property of a relation instance.
#[derive(Args, Debug, Clone)]
pub(crate) struct RelationInstanceIdAndPropertyArgs {
    /// The id of the relation instance.
    #[clap(flatten)]
    pub id: RelationInstanceIdArgs,

    /// The name of the property.
    pub property_name: String,
}

impl RelationInstanceIdAndPropertyArgs {
    pub fn not_found(&self) -> CommandError {
        self.id.not_found()
    }

    pub fn property_not_found(&self) -> CommandError {
        let id: RelationInstanceId = self.into();
        NotFound(format!("The relation instance with the id {} has no property {}", id, &self.property_name))
    }
}

impl From<&RelationInstanceIdAndPropertyArgs> for RelationInstanceId {
    fn from(args: &RelationInstanceIdAndPropertyArgs) -> Self {
        RelationInstanceId::from(&args.id)
    }
}
