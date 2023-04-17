use clap::Subcommand;

use crate::cli::system::command::args::ExecuteCommandArgs;
use crate::cli::system::plugin::args::PluginsArgs;
use crate::cli::types::components::args::ComponentsArgs;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum ClientCommands {
    // --- System ---
    /// Executes a command on the client.
    #[non_exhaustive]
    ExecuteCommand(ExecuteCommandArgs),

    /// Manage plugins.
    #[non_exhaustive]
    Plugins(PluginsArgs),

    // --- Types ---
    /// Manage components.
    #[non_exhaustive]
    Components(ComponentsArgs),
}
