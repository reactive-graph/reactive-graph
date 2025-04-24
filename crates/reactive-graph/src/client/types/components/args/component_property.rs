use crate::client::types::components::args::ComponentTypeIdArgs;
use clap::Args;
use reactive_graph_client::client::types::properties::variables::container::variables::PropertyContainerVariables;
use reactive_graph_graph::ComponentTypeId;

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
        let ty: ComponentTypeId = args.ty.clone().into();
        PropertyContainerVariables::new(ty, args.property_name.clone())
    }
}
