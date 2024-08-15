use crate::cli::types::entities::args::type_id::InboundEntityTypeIdArgs;
use crate::cli::types::entities::args::type_id::OutboundEntityTypeIdArgs;
use crate::cli::types::relations::args::type_id::RelationTypeIdArgs;
use clap::Args;
use reactive_graph_client::types::relations::create::queries::CreateRelationTypeVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateRelationTypeArgs {
    /// The outbound entity type.
    #[clap(flatten)]
    pub outbound_ty: OutboundEntityTypeIdArgs,

    /// The relation type.
    #[clap(flatten)]
    pub ty: RelationTypeIdArgs,

    /// The inbound entity type.
    #[clap(flatten)]
    pub inbound_ty: InboundEntityTypeIdArgs,

    /// The relation type description.
    pub description: Option<String>,
}

impl From<&CreateRelationTypeArgs> for CreateRelationTypeVariables {
    fn from(args: &CreateRelationTypeArgs) -> Self {
        CreateRelationTypeVariables {
            outbound_type_namespace: args.outbound_ty.outbound_type_namespace.clone(),
            outbound_type_name: args.outbound_ty.outbound_type_name.clone(),
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            inbound_type_namespace: args.inbound_ty.inbound_type_namespace.clone(),
            inbound_type_name: args.inbound_ty.inbound_type_name.clone(),
            description: args.description.clone(),
            properties: None,
            extensions: None,
        }
    }
}
