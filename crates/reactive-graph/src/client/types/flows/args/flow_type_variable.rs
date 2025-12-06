use crate::client::types::flows::args::parse_flow_ty;
use clap::Args;
use reactive_graph_graph::FlowTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct FlowTypeVariableArgs {
    /// The fully qualified namespace of the flow type.
    #[clap(name = "flow_type", value_parser = parse_flow_ty)]
    pub flow_ty: FlowTypeId,

    /// The name of the variable.
    pub variable_name: String,
}

// impl From<&FlowTypeVariableArgs> for PropertyContainerVariables {
//     fn from(args: &FlowTypeVariableArgs) -> Self {
//         let ty: FlowTypeId = args.ty.clone().into();
//         PropertyContainerVariables::new(ty, args.variable_name.clone())
//     }
// }
