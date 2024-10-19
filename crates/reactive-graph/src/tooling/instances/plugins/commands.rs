use crate::tooling::instances::plugins::args::InstallPluginArgs;
use crate::tooling::instances::plugins::args::UninstallPluginArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum PluginsCommands {
    /// Installs a plugin.
    Install(InstallPluginArgs),
    /// Uninstalls a plugin.
    Uninstall(UninstallPluginArgs),
}
