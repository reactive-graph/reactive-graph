use crate::client::types::components::args::add_extension::ComponentAddExtensionArgs;
use crate::client::types::components::args::add_property::ComponentAddPropertyArgs;
use crate::client::types::components::args::component_extension_type::ComponentExtensionTypeIdArgs;
use crate::client::types::components::args::component_property::ComponentPropertyArgs;
use crate::client::types::components::args::create::CreateComponentArgs;
use crate::client::types::components::args::parse_component_ty;
use crate::client::types::components::args::update_description::ComponentUpdateDescriptionArgs;
use clap::Subcommand;
use reactive_graph_graph::ComponentTypeId;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum ComponentsCommands {
    /// List all components.
    #[non_exhaustive]
    List,
    /// Prints a single component.
    #[non_exhaustive]
    Get {
        #[arg(name = "component", value_parser = parse_component_ty)]
        component_ty: ComponentTypeId,
    },
    /// List the properties of a component.
    #[non_exhaustive]
    ListProperties {
        #[arg(name = "component", value_parser = parse_component_ty)]
        component_ty: ComponentTypeId,
    },
    /// List the extensions of a component.
    #[non_exhaustive]
    ListExtensions {
        #[arg(name = "component", value_parser = parse_component_ty)]
        component_ty: ComponentTypeId,
    },
    /// Prints the JSON Schema of a component.
    #[non_exhaustive]
    GetJsonSchema {
        #[arg(name = "component", value_parser = parse_component_ty)]
        component_ty: ComponentTypeId,
    },
    /// Creates a new component.
    #[non_exhaustive]
    Create(CreateComponentArgs),
    /// Deletes a component.
    #[non_exhaustive]
    Delete {
        #[arg(name = "component", value_parser = parse_component_ty)]
        component_ty: ComponentTypeId,
    },
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
    /// Prints the JSON Schema of components.
    #[non_exhaustive]
    JsonSchema,
}
