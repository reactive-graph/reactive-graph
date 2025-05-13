use crate::client::instances::entities::args::EntityInstancesArgs;
use crate::client::instances::flows::args::FlowInstancesArgs;
use crate::client::instances::relations::args::RelationInstancesArgs;
use crate::client::introspection::args::IntrospectionQueryArgs;
use crate::client::system::command::args::ExecuteCommandArgs;
use crate::client::system::instance::args::InstanceInfoArgs;
use crate::client::system::plugin::args::PluginsArgs;
use crate::client::system::remotes::args::RemotesArgs;
use crate::client::types::components::args::ComponentsArgs;
use crate::client::types::entities::args::EntityTypesArgs;
use crate::client::types::flows::args::FlowTypesArgs;
use crate::client::types::relations::args::RelationTypesArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum ClientCommands {
    // --- System ---
    /// Executes a command on the client.
    #[non_exhaustive]
    ExecuteCommand(ExecuteCommandArgs),

    /// Prints information about the instance.
    #[non_exhaustive]
    InstanceInfo(InstanceInfoArgs),

    /// Manage plugins.
    #[non_exhaustive]
    Plugins(PluginsArgs),

    /// Manage remotes.
    #[non_exhaustive]
    Remotes(RemotesArgs),

    /// Shutdown the runtime.
    #[non_exhaustive]
    Shutdown,

    // --- Type System ---
    /// Manage components.
    #[non_exhaustive]
    Components(ComponentsArgs),

    /// Manage entity types.
    #[non_exhaustive]
    EntityTypes(EntityTypesArgs),

    /// Manage entity types.
    #[non_exhaustive]
    RelationTypes(RelationTypesArgs),

    /// Manage entity types.
    #[non_exhaustive]
    FlowTypes(FlowTypesArgs),

    // --- Instance System ---
    /// Manage entity instances.
    #[non_exhaustive]
    EntityInstances(EntityInstancesArgs),

    /// Manage relation instances.
    #[non_exhaustive]
    RelationInstances(RelationInstancesArgs),

    /// Manage flow instances.
    #[non_exhaustive]
    FlowInstances(FlowInstancesArgs),

    // --- Introspection ---
    /// Execute GraphQL introspection queries.
    #[non_exhaustive]
    Introspection(IntrospectionQueryArgs),
}
