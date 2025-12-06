use crate::client::types::flows::args::parse_flow_ty;
use crate::client::types::property_type::args::PropertyTypeDefinitionArgs;
use clap::Args;
use reactive_graph_graph::FlowTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct FlowTypeAddVariableArgs {
    /// The fully qualified namespace of the flow type.
    #[clap(name = "flow_type", value_parser = parse_flow_ty)]
    pub flow_ty: FlowTypeId,

    /// The variable.
    #[clap(flatten)]
    pub variable: PropertyTypeDefinitionArgs,
}
