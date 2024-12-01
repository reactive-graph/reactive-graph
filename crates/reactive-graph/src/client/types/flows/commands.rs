// use crate::client::types::flows::args::add_extension::FlowTypeAddExtensionArgs;
// use crate::client::types::flows::args::add_property::FlowTypeAddPropertyArgs;
// use crate::client::types::flows::args::create::CreateFlowTypeArgs;
// use crate::client::types::flows::args::entity_component_type::FlowComponentTypeIdArgs;
// use crate::client::types::flows::args::entity_extension_type::FlowExtensionTypeIdArgs;
// use crate::client::types::flows::args::entity_type_property::FlowTypePropertyArgs;
// use crate::client::types::flows::args::type_id::FlowTypeIdArgs;
// use crate::client::types::flows::args::update_description::FlowTypeUpdateDescriptionArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum FlowTypesCommands {
    // /// List all flow types.
    // #[non_exhaustive]
    // List,
    // /// Prints a single flow type.
    // #[non_exhaustive]
    // Get(FlowTypeIdArgs),
    // /// List the properties of a flow type.
    // #[non_exhaustive]
    // ListProperties(FlowTypeIdArgs),
    // /// List the extensions of a flow type.
    // #[non_exhaustive]
    // ListExtensions(FlowTypeIdArgs),
    // /// List the components of a flow type.
    // #[non_exhaustive]
    // ListComponents(FlowTypeIdArgs),
    // /// Creates a new flow type.
    // #[non_exhaustive]
    // Create(CreateFlowTypeArgs),
    // /// Deletes a flow type.
    // #[non_exhaustive]
    // Delete(FlowTypeIdArgs),
    // /// Adds a property to a flow type.
    // #[non_exhaustive]
    // AddProperty(FlowTypeAddPropertyArgs),
    // /// Removes a property from a flow type.
    // #[non_exhaustive]
    // RemoveProperty(FlowTypePropertyArgs),
    // /// Adds an extension to a flow type.
    // #[non_exhaustive]
    // AddExtension(FlowTypeAddExtensionArgs),
    // /// Removes an extension from a flow type.
    // #[non_exhaustive]
    // RemoveExtension(FlowExtensionTypeIdArgs),
    // /// Adds a component to a flow type.
    // #[non_exhaustive]
    // AddComponent(FlowComponentTypeIdArgs),
    // /// Removes a component from a flow type.
    // #[non_exhaustive]
    // RemoveComponent(FlowComponentTypeIdArgs),
    // /// Updates the description of a flow type.
    // #[non_exhaustive]
    // UpdateDescription(FlowTypeUpdateDescriptionArgs),
    /// Prints the JSON Schema of flow types.
    #[non_exhaustive]
    JsonSchema,
}
