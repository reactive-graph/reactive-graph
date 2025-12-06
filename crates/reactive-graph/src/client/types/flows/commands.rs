use crate::client::types::flows::args::add_entity_instance::FlowTypeAddEntityInstanceArgs;
use crate::client::types::flows::args::add_extension::FlowTypeAddExtensionArgs;
use crate::client::types::flows::args::add_variable::FlowTypeAddVariableArgs;
use crate::client::types::flows::args::create::CreateFlowTypeArgs;
use crate::client::types::flows::args::flow_extension_type::FlowExtensionTypeIdArgs;
use crate::client::types::flows::args::flow_type_variable::FlowTypeVariableArgs;
use crate::client::types::flows::args::parse_flow_ty;
use crate::client::types::flows::args::remove_entity_instance::FlowTypeRemoveEntityInstanceArgs;
use crate::client::types::flows::args::update_description::FlowTypeUpdateDescriptionArgs;
use clap::Subcommand;
use reactive_graph_graph::FlowTypeId;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum FlowTypesCommands {
    /// List all flow types.
    #[non_exhaustive]
    List,
    /// Prints a single flow type.
    #[non_exhaustive]
    Get {
        #[arg(name = "flow_type", value_parser = parse_flow_ty)]
        flow_ty: FlowTypeId,
    },
    /// List the variables of a flow type.
    #[non_exhaustive]
    ListVariables {
        #[arg(name = "flow_type", value_parser = parse_flow_ty)]
        flow_ty: FlowTypeId,
    },
    /// List the extensions of a flow type.
    #[non_exhaustive]
    ListExtensions {
        #[arg(name = "flow_type", value_parser = parse_flow_ty)]
        flow_ty: FlowTypeId,
    },
    /// Prints the JSON Schema of a flow type.
    #[non_exhaustive]
    GetJsonSchema {
        #[arg(name = "flow_type", value_parser = parse_flow_ty)]
        flow_ty: FlowTypeId,
    },
    /// Creates a new flow type.
    #[non_exhaustive]
    Create(CreateFlowTypeArgs),
    /// Deletes a flow type.
    #[non_exhaustive]
    Delete {
        #[arg(name = "flow_type", value_parser = parse_flow_ty)]
        flow_ty: FlowTypeId,
    },
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
