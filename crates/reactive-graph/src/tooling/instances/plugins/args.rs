use crate::tooling::instances::plugins::commands::PluginsCommands;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct PluginsArgs {
    #[command(subcommand)]
    pub commands: PluginsCommands,
}

#[derive(Parser, Debug)]
pub struct InstallPluginArgs {
    /// The name of the plugin.
    pub plugin_name: String,
}

#[derive(Parser, Debug)]
pub struct UninstallPluginArgs {
    /// The name of the plugin.
    pub plugin_name: String,
}
