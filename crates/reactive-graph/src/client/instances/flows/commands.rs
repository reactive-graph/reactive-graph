use crate::client::instances::flows::args::create_from_type::CreateFlowInstanceFromTypeArgs;
use crate::client::instances::flows::args::id::IdArgs;
use crate::client::instances::flows::args::label::LabelArgs;
use crate::client::instances::flows::args::search::SearchFlowInstancesArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum FlowInstancesCommands {
    /// List all flow instances.
    #[non_exhaustive]
    List(SearchFlowInstancesArgs),
    /// Prints a single flow instance.
    #[non_exhaustive]
    Get(IdArgs),
    /// Prints a single flow instance.
    #[non_exhaustive]
    GetByLabel(LabelArgs),

    //
    // /// Creates a new flow and a corresponding wrapper entity instance.
    // #[non_exhaustive]
    // Create(CreateFlowInstanceArgs),
    //
    /// Creates a new flow from the given type.
    #[non_exhaustive]
    CreateFromType(CreateFlowInstanceFromTypeArgs),

    //
    // TODO: create_entity
    // TODO: add_entity
    // TODO: remove_entity
    //
    // TODO: create_relation
    // TODO: add_relation
    // TODO: remove_relation
    //

    // Deletes a flow instance.
    #[non_exhaustive]
    Delete(IdArgs),
    /// Prints the JSON Schema of flow instances.
    #[non_exhaustive]
    JsonSchema,
}
