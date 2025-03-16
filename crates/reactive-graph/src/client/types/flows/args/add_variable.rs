use crate::client::types::flows::args::type_id::FlowTypeIdArgs;
use crate::client::types::property_type::args::PropertyTypeDefinitionArgs;
use clap::Args;
use reactive_graph_client::PropertyTypeDefinition;
use reactive_graph_client::types::flows::add_variable::queries::AddVariableVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct FlowTypeAddVariableArgs {
    /// The flow type.
    #[clap(flatten)]
    pub ty: FlowTypeIdArgs,

    /// The variable.
    #[clap(flatten)]
    pub variable: PropertyTypeDefinitionArgs,
}

impl From<&FlowTypeAddVariableArgs> for AddVariableVariables {
    fn from(args: &FlowTypeAddVariableArgs) -> Self {
        AddVariableVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            variable: PropertyTypeDefinition {
                name: args.variable.property_name.clone(),
                description: args.variable.description.clone().unwrap_or_default(),
                data_type: args.variable.data_type.into(),
                socket_type: args.variable.socket_type.into(),
                mutability: args.variable.mutability.into(),
                extensions: Vec::new(),
            },
        }
    }
}
