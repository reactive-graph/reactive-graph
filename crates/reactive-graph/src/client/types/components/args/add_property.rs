use crate::client::types::components::args::ComponentTypeIdArgs;
use crate::client::types::property_type::args::PropertyTypeDefinitionArgs;
use clap::Args;
use reactive_graph_client::PropertyTypeDefinition;
use reactive_graph_client::types::components::add_property::queries::AddPropertyVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentAddPropertyArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ComponentTypeIdArgs,

    /// The property.
    #[clap(flatten)]
    pub property_type: PropertyTypeDefinitionArgs,
}

impl From<&ComponentAddPropertyArgs> for AddPropertyVariables {
    fn from(args: &ComponentAddPropertyArgs) -> Self {
        AddPropertyVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            property: PropertyTypeDefinition {
                name: args.property_type.property_name.clone(),
                description: args.property_type.description.clone().unwrap_or_default(),
                data_type: args.property_type.data_type.into(),
                socket_type: args.property_type.socket_type.into(),
                mutability: args.property_type.mutability.into(),
                extensions: Vec::new(),
            },
        }
    }
}
