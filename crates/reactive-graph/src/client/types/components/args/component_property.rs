use crate::client::types::components::args::ComponentTypeIdArgs;
use clap::Args;
use reactive_graph_client::client::types::properties::container::queries::PropertyContainerVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentPropertyArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ComponentTypeIdArgs,

    /// The name of the property.
    pub property_name: String,
}

impl From<&ComponentPropertyArgs> for PropertyContainerVariables {
    fn from(args: &ComponentPropertyArgs) -> Self {
        PropertyContainerVariables::builder()
            .namespace(args.ty.namespace.clone())
            .name(args.ty.name.clone())
            .property_name(args.property_name.clone())
            .build()
    }
}
