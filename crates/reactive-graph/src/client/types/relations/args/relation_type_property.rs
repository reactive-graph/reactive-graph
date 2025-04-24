use crate::client::types::relations::args::type_id::RelationTypeIdArgs;
use clap::Args;
use reactive_graph_client::client::types::properties::variables::container::variables::PropertyContainerVariables;
use reactive_graph_graph::RelationTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct RelationTypePropertyArgs {
    /// The relation type.
    #[clap(flatten)]
    pub ty: RelationTypeIdArgs,

    /// The name of the property.
    pub property_name: String,
}

impl From<&RelationTypePropertyArgs> for PropertyContainerVariables {
    fn from(args: &RelationTypePropertyArgs) -> Self {
        let ty: RelationTypeId = args.ty.clone().into();
        PropertyContainerVariables::new(ty, args.property_name.clone())
    }
}
