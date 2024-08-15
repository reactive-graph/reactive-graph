use crate::cli::types::entities::args::type_id::EntityTypeIdArgs;
use clap::Args;
use reactive_graph_client::client::types::properties::container::queries::PropertyContainerVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct EntityTypePropertyArgs {
    /// The entity type.
    #[clap(flatten)]
    pub ty: EntityTypeIdArgs,

    /// The name of the property.
    pub property_name: String,
}

impl From<&EntityTypePropertyArgs> for PropertyContainerVariables {
    fn from(args: &EntityTypePropertyArgs) -> Self {
        PropertyContainerVariables::builder()
            .namespace(args.ty.namespace.clone())
            .name(args.ty.name.clone())
            .property_name(args.property_name.clone())
            .build()
    }
}
