use clap::Subcommand;

use crate::cli::system::plugin::args::PluginByNameArgs;
use crate::cli::system::plugin::args::SearchPluginsArgs;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum PluginsCommands {
    /// Lists all plugins.
    #[non_exhaustive]
    List,
    /// Search for plugins by name, state or stem.
    #[non_exhaustive]
    Search(SearchPluginsArgs),
    /// Prints a single plugin.
    #[non_exhaustive]
    Get(PluginByNameArgs),
    /// Depends on.
    #[non_exhaustive]
    Dependencies(PluginByNameArgs),
    /// Dependent plugins.
    #[non_exhaustive]
    Dependents(PluginByNameArgs),
    /// Starts a plugin.
    #[non_exhaustive]
    Start(PluginByNameArgs),
    /// Stops a plugin.
    #[non_exhaustive]
    Stop(PluginByNameArgs),
    /// Restarts a plugin.
    #[non_exhaustive]
    Restart(PluginByNameArgs),
}
