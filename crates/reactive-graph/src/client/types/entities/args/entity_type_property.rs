use crate::client::types::entities::args::type_id::EntityTypeIdArgs;
use clap::Args;
use reactive_graph_client::client::types::properties::variables::container::variables::PropertyContainerVariables;
use reactive_graph_graph::EntityTypeId;

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
        let ty: EntityTypeId = args.ty.clone().into();
        PropertyContainerVariables::new(ty, args.property_name.clone())
    }
}
