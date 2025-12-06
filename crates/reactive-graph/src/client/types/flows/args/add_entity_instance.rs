use crate::client::instances::flows::args::add_entity_instance::AddEntityInstanceArgs;
use crate::client::types::flows::args::parse_flow_ty;
use clap::Args;
use reactive_graph_graph::FlowTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct FlowTypeAddEntityInstanceArgs {
    /// The fully qualified namespace of the flow type.
    #[clap(name = "flow_type", value_parser = parse_flow_ty)]
    pub flow_ty: FlowTypeId,

    /// The entity instance to add.
    #[clap(flatten)]
    pub entity_instance: AddEntityInstanceArgs,
}
