use crate::tooling::instances::plugins::args::PluginsArgs;
use crate::tooling::instances::plugins::commands::PluginsCommands;
use crate::tooling::instances::plugins::install::install_plugin;
use crate::tooling::instances::plugins::uninstall::uninstall_plugin;
use anyhow::Result;
use std::path::Path;

pub mod args;
pub mod commands;
pub mod install;
pub mod uninstall;

pub fn handle_plugins(instance_dir: &Path, args: PluginsArgs) -> Result<()> {
    match args.commands {
        PluginsCommands::Install(args) => install_plugin(instance_dir, args)?,
        PluginsCommands::Uninstall(args) => uninstall_plugin(instance_dir, args)?,
    }
    Ok(())
}
