use clap::Subcommand;

use crate::cli::types::components::args::add_extension::ComponentAddExtensionArgs;
use crate::cli::types::components::args::add_property::ComponentAddPropertyArgs;
use crate::cli::types::components::args::component_extension_type::ComponentExtensionTypeIdArgs;
use crate::cli::types::components::args::component_property::ComponentPropertyArgs;
use crate::cli::types::components::args::create::CreateComponentArgs;
use crate::cli::types::components::args::type_id::ComponentTypeIdArgs;
use crate::cli::types::components::args::update_description::ComponentUpdateDescriptionArgs;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum ComponentsCommands {
    /// List all components.
    #[non_exhaustive]
    List,
    /// Prints a single component.
    #[non_exhaustive]
    Get(ComponentTypeIdArgs),
    /// List the properties of a component.
    #[non_exhaustive]
    ListProperties(ComponentTypeIdArgs),
    /// List the extensions of a component.
    #[non_exhaustive]
    ListExtensions(ComponentTypeIdArgs),
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
    RemoveProperty(ComponentPropertyArgs),
    /// Adds an extension to a component.
    #[non_exhaustive]
    AddExtension(ComponentAddExtensionArgs),
    /// Removes an extension from a component.
    #[non_exhaustive]
    RemoveExtension(ComponentExtensionTypeIdArgs),
    /// Updates the description of a component.
    #[non_exhaustive]
    UpdateDescription(ComponentUpdateDescriptionArgs),
}
