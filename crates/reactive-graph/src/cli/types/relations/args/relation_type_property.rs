use crate::cli::types::relations::args::type_id::RelationTypeIdArgs;
use clap::Args;
use reactive_graph_client::client::types::properties::container::queries::PropertyContainerVariables;

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
        PropertyContainerVariables::builder()
            .namespace(args.ty.namespace.clone())
            .name(args.ty.name.clone())
            .property_name(args.property_name.clone())
            .build()
    }
}
