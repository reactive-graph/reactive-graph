use crate::cli::types::entities::args::type_id::EntityTypeIdArgs;
use clap::Args;
use reactive_graph_client::types::entity_types::create::queries::CreateEntityTypeVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateEntityTypeArgs {
    /// The entity type.
    #[clap(flatten)]
    pub ty: EntityTypeIdArgs,

    /// The entity type description.
    pub description: Option<String>,
}

impl From<&CreateEntityTypeArgs> for CreateEntityTypeVariables {
    fn from(args: &CreateEntityTypeArgs) -> Self {
        CreateEntityTypeVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            description: args.description.clone(),
            properties: None,
            extensions: None,
        }
    }
}
