use clap::Args;

use crate::cli::types::entities::commands::EntityTypesCommands;
use reactive_graph_graph::EntityTypeId;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct EntityTypesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<EntityTypesCommands>,
}

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateEntityTypeArgs {
    /// The entity type type.
    #[clap(flatten)]
    pub ty: EntityTypeIdArgs,

    /// The entity type description.
    pub description: Option<String>,
}

/// The component type.
#[derive(Args, Debug, Clone)]
pub(crate) struct EntityTypeIdArgs {
    /// The entity type namespace.
    pub namespace: String,

    /// The entity type name.
    pub name: String,
}

impl From<EntityTypeIdArgs> for EntityTypeId {
    fn from(ty: EntityTypeIdArgs) -> Self {
        EntityTypeId::new_from_type(ty.namespace, ty.name)
    }
}
