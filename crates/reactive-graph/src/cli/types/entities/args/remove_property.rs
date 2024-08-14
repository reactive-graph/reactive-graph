use crate::cli::types::entities::args::type_id::EntityTypeIdArgs;
use clap::Args;
use reactive_graph_client::types::entity_types::remove_property::queries::RemovePropertyVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct EntityTypeRemovePropertyArgs {
    /// The entity type.
    #[clap(flatten)]
    pub ty: EntityTypeIdArgs,

    /// The name of the property.
    pub property_name: String,
}

impl From<&EntityTypeRemovePropertyArgs> for RemovePropertyVariables {
    fn from(args: &EntityTypeRemovePropertyArgs) -> Self {
        RemovePropertyVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            property_name: args.property_name.clone(),
        }
    }
}
