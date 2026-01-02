use crate::client::types::flows::args::parse_flow_ty;
use clap::Args;
use reactive_graph_graph::FlowTypeId;
use uuid::Uuid;

#[derive(Args, Debug, Clone)]
pub(crate) struct FlowTypeRemoveEntityInstanceArgs {
    /// The fully qualified namespace of the flow type.
    #[clap(name = "flow_type", value_parser = parse_flow_ty)]
    pub flow_ty: FlowTypeId,

    /// The entity instance to remove from the flow type.
    pub id: Uuid,
}

// impl From<&FlowTypeRemoveEntityInstanceArgs> for RemoveEntityInstanceVariables {
//     fn from(args: &FlowTypeRemoveEntityInstanceArgs) -> Self {
//         RemoveEntityInstanceVariables {
//             namespace: args.ty.namespace.clone(),
//             name: args.ty.name.clone(),
//             id: args.id.into(),
//         }
//     }
// }
