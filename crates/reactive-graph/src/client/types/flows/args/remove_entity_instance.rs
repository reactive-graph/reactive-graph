use crate::client::types::flows::args::type_id::FlowTypeIdArgs;
use clap::Args;
use reactive_graph_client::types::flows::variables::remove_entity_instance::variables::RemoveEntityInstanceVariables;
use uuid::Uuid;

#[derive(Args, Debug, Clone)]
pub(crate) struct FlowTypeRemoveEntityInstanceArgs {
    /// The flow type.
    #[clap(flatten)]
    pub ty: FlowTypeIdArgs,

    /// The entity instance to remove from the flow type.
    pub id: Uuid,
}

impl From<&FlowTypeRemoveEntityInstanceArgs> for RemoveEntityInstanceVariables {
    fn from(args: &FlowTypeRemoveEntityInstanceArgs) -> Self {
        RemoveEntityInstanceVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            id: args.id.into(),
        }
    }
}
