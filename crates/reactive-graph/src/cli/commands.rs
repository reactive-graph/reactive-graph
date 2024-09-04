use crate::cli::instances::entities::args::EntityInstancesArgs;
use crate::cli::instances::relations::args::RelationInstancesArgs;
use crate::cli::system::command::args::ExecuteCommandArgs;
use crate::cli::system::instance::args::InstanceInfoArgs;
use crate::cli::system::plugin::args::PluginsArgs;
use crate::cli::system::remotes::args::RemotesArgs;
use crate::cli::types::components::args::ComponentsArgs;
use crate::cli::types::entities::args::EntityTypesArgs;
use crate::cli::types::relations::args::RelationTypesArgs;
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

    // --- Types ---
    /// Manage components.
    #[non_exhaustive]
    Components(ComponentsArgs),

    /// Manage entity types.
    #[non_exhaustive]
    EntityTypes(EntityTypesArgs),

    /// Manage entity types.
    #[non_exhaustive]
    RelationTypes(RelationTypesArgs),

    // --- Instances ---
    /// Manage entity instances.
    #[non_exhaustive]
    EntityInstances(EntityInstancesArgs),

    /// Manage relation instances.
    #[non_exhaustive]
    RelationInstances(RelationInstancesArgs),
}
