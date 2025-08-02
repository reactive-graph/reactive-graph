use crate::tooling::instances::plugins::install::args::InstallPluginsFromRepositoryArgs;
use crate::tooling::instances::plugins::uninstall::args::UninstallPluginArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum PluginsCommands {
    /// Installs plugins from a plugin repository.
    Install(InstallPluginsFromRepositoryArgs),
    /// Uninstalls a plugin.
    Uninstall(UninstallPluginArgs),
}
