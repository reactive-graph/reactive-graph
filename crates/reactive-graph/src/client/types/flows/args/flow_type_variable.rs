use crate::client::types::flows::args::type_id::FlowTypeIdArgs;
use clap::Args;
use reactive_graph_client::client::types::properties::variables::container::variables::PropertyContainerVariables;
use reactive_graph_graph::FlowTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct FlowTypeVariableArgs {
    /// The flow type.
    #[clap(flatten)]
    pub ty: FlowTypeIdArgs,

    /// The name of the variable.
    pub variable_name: String,
}

impl From<&FlowTypeVariableArgs> for PropertyContainerVariables {
    fn from(args: &FlowTypeVariableArgs) -> Self {
        let ty: FlowTypeId = args.ty.clone().into();
        PropertyContainerVariables::new(ty, args.variable_name.clone())
    }
}
