use clap::Subcommand;

use crate::cli::types::components::args::ComponentTypeIdArgs;
use crate::cli::types::components::args::CreateComponentArgs;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum ComponentsCommands {
    /// List all components.
    #[non_exhaustive]
    List,
    /// Prints a single component.
    #[non_exhaustive]
    Get(ComponentTypeIdArgs),
    /// Creates a new component.
    #[non_exhaustive]
    Create(CreateComponentArgs),
}
