use crate::cli::error::CommandError;
use crate::cli::instances::relations::args::id::RelationInstanceIdArgs;
use crate::cli::types::components::args::type_id::ComponentTypeIdArgs;
use clap::Args;
use reactive_graph_graph::RelationInstanceId;

/// Identifies a component of a relation instance.
#[derive(Args, Debug, Clone)]
pub(crate) struct RelationInstanceIdAndComponentArgs {
    /// The id of the relation instance.
    #[clap(flatten)]
    pub id: RelationInstanceIdArgs,

    /// The component type of the reactive instance.
    #[clap(flatten)]
    pub component_ty: ComponentTypeIdArgs,
}

impl RelationInstanceIdAndComponentArgs {
    pub fn not_found(&self) -> CommandError {
        self.id.not_found()
    }
}

impl From<&RelationInstanceIdAndComponentArgs> for RelationInstanceId {
    fn from(args: &RelationInstanceIdAndComponentArgs) -> Self {
        RelationInstanceId::from(&args.id)
    }
}