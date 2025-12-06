use crate::client::types::flows::args::parse_flow_ty;
use clap::Args;
use reactive_graph_graph::FlowTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct FlowTypeUpdateDescriptionArgs {
    /// The fully qualified namespace of the flow type.
    #[clap(name = "flow_type", value_parser = parse_flow_ty)]
    pub flow_ty: FlowTypeId,

    /// The description to update.
    pub description: String,
}
