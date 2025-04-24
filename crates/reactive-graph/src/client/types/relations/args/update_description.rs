use crate::client::types::relations::args::type_id::RelationTypeIdArgs;
use clap::Args;
use reactive_graph_client::types::common::variables::update_description::variables::UpdateDescriptionVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct RelationTypeUpdateDescriptionArgs {
    /// The relation type.
    #[clap(flatten)]
    pub ty: RelationTypeIdArgs,

    /// The description to update.
    pub description: String,
}

impl From<&RelationTypeUpdateDescriptionArgs> for UpdateDescriptionVariables {
    fn from(args: &RelationTypeUpdateDescriptionArgs) -> Self {
        UpdateDescriptionVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            description: args.description.clone(),
        }
    }
}
