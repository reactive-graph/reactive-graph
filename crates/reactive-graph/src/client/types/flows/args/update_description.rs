use crate::client::types::flows::args::type_id::FlowTypeIdArgs;
use clap::Args;
use reactive_graph_client::types::flows::update_description::queries::UpdateDescriptionVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct FlowTypeUpdateDescriptionArgs {
    /// The flow type.
    #[clap(flatten)]
    pub ty: FlowTypeIdArgs,

    /// The description to update.
    pub description: String,
}

impl From<&FlowTypeUpdateDescriptionArgs> for UpdateDescriptionVariables {
    fn from(args: &FlowTypeUpdateDescriptionArgs) -> Self {
        UpdateDescriptionVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            description: args.description.clone(),
        }
    }
}
