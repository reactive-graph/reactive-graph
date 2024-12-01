// use crate::client::instances::flows::args::add_property::AddPropertyArgs;
// use crate::client::instances::flows::args::create::CreateFlowInstanceArgs;
// use crate::client::instances::flows::args::id::FlowInstanceIdArgs;
// use crate::client::instances::flows::args::id_and_component::FlowInstanceIdAndComponentArgs;
// use crate::client::instances::flows::args::id_and_property::FlowInstanceIdAndPropertyArgs;
// use crate::client::instances::flows::args::search::SearchFlowInstancesArgs;
// use crate::client::instances::flows::args::set_property::SetPropertyArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum FlowInstancesCommands {
    // /// List all flow instances.
    // #[non_exhaustive]
    // List(SearchFlowInstancesArgs),
    // /// Prints a single flow instance.
    // #[non_exhaustive]
    // Get(FlowInstanceIdArgs),
    // /// Lists the properties of a flow instance.
    // #[non_exhaustive]
    // ListProperties(FlowInstanceIdArgs),
    // /// Prints the value of a property of a flow instance.
    // #[non_exhaustive]
    // GetProperty(FlowInstanceIdAndPropertyArgs),
    // /// Sets the value of a property of a flow instance.
    // #[non_exhaustive]
    // SetProperty(SetPropertyArgs),
    // /// Adds a new property to a flow instance.
    // #[non_exhaustive]
    // AddProperty(AddPropertyArgs),
    // /// Removes a property from a flow instance.
    // #[non_exhaustive]
    // RemoveProperty(FlowInstanceIdAndPropertyArgs),
    // /// Lists the components of a flow instance.
    // #[non_exhaustive]
    // ListComponents(FlowInstanceIdArgs),
    // /// Adds a component to a flow instance.
    // #[non_exhaustive]
    // AddComponent(FlowInstanceIdAndComponentArgs),
    // /// Removes a component from a flow instance.
    // #[non_exhaustive]
    // RemoveComponent(FlowInstanceIdAndComponentArgs),
    // /// Creates a new flow type.
    // #[non_exhaustive]
    // Create(CreateFlowInstanceArgs),
    // // Deletes a flow instance.
    // #[non_exhaustive]
    // Delete(FlowInstanceIdArgs),
    /// Prints the JSON Schema of flow instances.
    #[non_exhaustive]
    JsonSchema,
}
