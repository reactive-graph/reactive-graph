use crate::client::types::components::args::type_id::ComponentTypeIdArgs;
use clap::Args;
use reactive_graph_client::types::components::variables::create::variables::CreateComponentVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateComponentArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ComponentTypeIdArgs,

    /// The component description.
    pub description: Option<String>,
}

impl From<&CreateComponentArgs> for CreateComponentVariables {
    fn from(args: &CreateComponentArgs) -> Self {
        CreateComponentVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            description: args.description.clone(),
            properties: None,
            extensions: None,
        }
    }
}
