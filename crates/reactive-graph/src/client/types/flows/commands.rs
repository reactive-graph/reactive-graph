use crate::client::types::flows::args::add_entity_instance::FlowTypeAddEntityInstanceArgs;
use crate::client::types::flows::args::add_extension::FlowTypeAddExtensionArgs;
use crate::client::types::flows::args::add_variable::FlowTypeAddVariableArgs;
use crate::client::types::flows::args::create::CreateFlowTypeArgs;
use crate::client::types::flows::args::flow_extension_type::FlowExtensionTypeIdArgs;
use crate::client::types::flows::args::flow_type_variable::FlowTypeVariableArgs;
use crate::client::types::flows::args::remove_entity_instance::FlowTypeRemoveEntityInstanceArgs;
use crate::client::types::flows::args::type_id::FlowTypeIdArgs;
use crate::client::types::flows::args::update_description::FlowTypeUpdateDescriptionArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum FlowTypesCommands {
    /// List all flow types.
    #[non_exhaustive]
    List,
    /// Prints a single flow type.
    #[non_exhaustive]
    Get(FlowTypeIdArgs),
    /// List the variables of a flow type.
    #[non_exhaustive]
    ListVariables(FlowTypeIdArgs),
    /// List the extensions of a flow type.
    #[non_exhaustive]
    ListExtensions(FlowTypeIdArgs),
    /// Creates a new flow type.
    #[non_exhaustive]
    Create(CreateFlowTypeArgs),
    /// Deletes a flow type.
    #[non_exhaustive]
    Delete(FlowTypeIdArgs),
    /// Adds a property to a flow type.
    #[non_exhaustive]
    AddVariable(FlowTypeAddVariableArgs),
    /// Removes a property from a flow type.
    #[non_exhaustive]
    RemoveVariable(FlowTypeVariableArgs),
    /// Adds an extension to a flow type.
    #[non_exhaustive]
    AddExtension(FlowTypeAddExtensionArgs),
    /// Removes an extension from a flow type.
    #[non_exhaustive]
    RemoveExtension(FlowExtensionTypeIdArgs),
    /// Updates the description of a flow type.
    #[non_exhaustive]
    UpdateDescription(FlowTypeUpdateDescriptionArgs),
    /// Adds a new entity instance to a flow type.
    #[non_exhaustive]
    AddEntityInstance(FlowTypeAddEntityInstanceArgs),
    /// Removes an entity instance to a flow type.
    #[non_exhaustive]
    RemoveEntityInstance(FlowTypeRemoveEntityInstanceArgs),
    /// Prints the JSON Schema of flow types.
    #[non_exhaustive]
    JsonSchema,
}
