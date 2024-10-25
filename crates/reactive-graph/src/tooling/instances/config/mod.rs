use crate::tooling::instances::config::args::ConfigInstanceArgs;
use crate::tooling::instances::config::commands::InstanceConfigCommands;
use crate::tooling::instances::config::graphql::handle_graphql_config;
use crate::tooling::instances::config::instance::handle_instance_config;
use crate::tooling::instances::config::plugins::handle_plugins_config;
use std::path::PathBuf;

pub mod args;
pub mod commands;
pub mod graphql;
pub mod instance;
pub mod plugins;
pub mod serde;

pub const CONFIG_DIR_NAME: &str = "config";

pub fn handle_config(instance_dir: &PathBuf, args: ConfigInstanceArgs) -> anyhow::Result<()> {
    let config_dir = get_config_dir(instance_dir);
    match args.commands {
        InstanceConfigCommands::Graphql(args) => handle_graphql_config(&config_dir, args),
        InstanceConfigCommands::Instance(args) => handle_instance_config(&config_dir, args),
        InstanceConfigCommands::Plugins(args) => handle_plugins_config(&config_dir, args),
    }
}

pub fn get_config_dir(instance_dir: &PathBuf) -> PathBuf {
    let mut config_dir = instance_dir.to_owned();
    config_dir.push(CONFIG_DIR_NAME);
    config_dir
}
