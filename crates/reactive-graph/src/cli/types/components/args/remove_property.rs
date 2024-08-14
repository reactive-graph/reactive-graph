use crate::cli::types::components::args::ComponentTypeIdArgs;
use clap::Args;
use reactive_graph_client::types::components::remove_property::queries::RemovePropertyVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentRemovePropertyArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ComponentTypeIdArgs,

    /// The name of the property.
    pub property_name: String,
}

impl From<&ComponentRemovePropertyArgs> for RemovePropertyVariables {
    fn from(args: &ComponentRemovePropertyArgs) -> Self {
        RemovePropertyVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            property_name: args.property_name.clone(),
        }
    }
}
