use crate::client::types::components::args::ComponentTypeIdArgs;
use clap::Args;
use reactive_graph_client::types::common::variables::update_description::variables::UpdateDescriptionVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentUpdateDescriptionArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ComponentTypeIdArgs,

    /// The description to update.
    pub description: String,
}

impl From<&ComponentUpdateDescriptionArgs> for UpdateDescriptionVariables {
    fn from(args: &ComponentUpdateDescriptionArgs) -> Self {
        UpdateDescriptionVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            description: args.description.clone(),
        }
    }
}
