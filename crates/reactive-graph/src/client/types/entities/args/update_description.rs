use crate::client::types::entities::args::type_id::EntityTypeIdArgs;
use clap::Args;
use reactive_graph_client::types::common::variables::update_description::variables::UpdateDescriptionVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct EntityTypeUpdateDescriptionArgs {
    /// The entity type.
    #[clap(flatten)]
    pub ty: EntityTypeIdArgs,

    /// The description to update.
    pub description: String,
}

impl From<&EntityTypeUpdateDescriptionArgs> for UpdateDescriptionVariables {
    fn from(args: &EntityTypeUpdateDescriptionArgs) -> Self {
        UpdateDescriptionVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            description: args.description.clone(),
        }
    }
}
