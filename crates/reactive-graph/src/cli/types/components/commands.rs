use clap::Subcommand;

use crate::cli::types::components::args::ComponentAddExtensionArgs;
use crate::cli::types::components::args::ComponentAddPropertyArgs;
use crate::cli::types::components::args::ComponentRemoveExtensionArgs;
use crate::cli::types::components::args::ComponentRemovePropertyArgs;
use crate::cli::types::components::args::ComponentTypeIdArgs;
use crate::cli::types::components::args::ComponentUpdateDescriptionArgs;
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
    /// Deletes a component.
    #[non_exhaustive]
    Delete(ComponentTypeIdArgs),
    /// Adds a property to a component.
    #[non_exhaustive]
    AddProperty(ComponentAddPropertyArgs),
    /// Removes a property from a component.
    #[non_exhaustive]
    RemoveProperty(ComponentRemovePropertyArgs),
    /// Adds an extension to a component.
    #[non_exhaustive]
    AddExtension(ComponentAddExtensionArgs),
    /// Removes an extension from a component.
    #[non_exhaustive]
    RemoveExtension(ComponentRemoveExtensionArgs),
    /// Updates the description of a component.
    #[non_exhaustive]
    UpdateDescription(ComponentUpdateDescriptionArgs),
}
