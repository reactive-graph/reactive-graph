use crate::client::types::entities::args::type_id::EntityTypeIdArgs;
use crate::client::types::property_type::args::PropertyTypeDefinitionArgs;
use clap::Args;
use reactive_graph_client::types::entities::add_property::queries::AddPropertyVariables;
use reactive_graph_client::PropertyTypeDefinition;

#[derive(Args, Debug, Clone)]
pub(crate) struct EntityTypeAddPropertyArgs {
    /// The entity type.
    #[clap(flatten)]
    pub ty: EntityTypeIdArgs,

    /// The property.
    #[clap(flatten)]
    pub property_type: PropertyTypeDefinitionArgs,
}

impl From<&EntityTypeAddPropertyArgs> for AddPropertyVariables {
    fn from(args: &EntityTypeAddPropertyArgs) -> Self {
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
