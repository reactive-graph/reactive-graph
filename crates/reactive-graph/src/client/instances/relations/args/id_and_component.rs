use crate::client::error::CommandError;
use crate::client::instances::relations::args::id::RelationInstanceIdArgs;
use crate::client::types::components::args::parse_component_ty;
use clap::Args;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::RelationInstanceId;

/// Identifies a component of a relation instance.
#[derive(Args, Debug, Clone)]
pub(crate) struct RelationInstanceIdAndComponentArgs {
    /// The id of the relation instance.
    #[clap(flatten)]
    pub id: RelationInstanceIdArgs,

    /// The fully qualified namespace of the component.
    #[clap(name = "component", value_parser = parse_component_ty)]
    pub component_ty: ComponentTypeId,
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
