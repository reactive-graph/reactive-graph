use crate::client::types::flows::args::type_id::FlowTypeIdArgs;
use clap::Args;
use reactive_graph_client::client::types::properties::container::queries::PropertyContainerVariables;

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
        PropertyContainerVariables::builder()
            .namespace(args.ty.namespace.clone())
            .name(args.ty.name.clone())
            .property_name(args.variable_name.clone())
            .build()
    }
}
