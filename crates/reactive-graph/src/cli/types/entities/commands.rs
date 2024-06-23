use clap::Subcommand;

use crate::cli::types::entities::args::CreateEntityTypeArgs;
use crate::cli::types::entities::args::EntityTypeIdArgs;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum EntityTypesCommands {
    /// List all entity types.
    #[non_exhaustive]
    List,
    /// Prints a single entity type.
    #[non_exhaustive]
    Get(EntityTypeIdArgs),
    /// Creates a new entity type.
    #[non_exhaustive]
    Create(CreateEntityTypeArgs),
}
