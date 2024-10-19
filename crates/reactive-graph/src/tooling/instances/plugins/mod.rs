use crate::tooling::instances::plugins::args::PluginsArgs;
use crate::tooling::instances::plugins::commands::PluginsCommands;
use crate::tooling::instances::plugins::install::install_plugin;
use crate::tooling::instances::plugins::uninstall::uninstall_plugin;
use std::path::PathBuf;

pub mod args;
pub mod commands;
pub mod install;
pub mod uninstall;

pub fn handle_plugins(instance_dir: &PathBuf, args: PluginsArgs) {
    match args.commands {
        PluginsCommands::Install(args) => {
            install_plugin(instance_dir, args);
        }
        PluginsCommands::Uninstall(args) => {
            uninstall_plugin(instance_dir, args);
        }
    }
}
