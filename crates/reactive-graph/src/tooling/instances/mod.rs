pub mod args;
pub mod commands;
pub mod create_dir;
pub mod init;
pub mod plugins;
pub mod repositories;

use crate::tooling::instances::args::InstancesArgs;
use crate::tooling::instances::commands::InstancesCommands;
use crate::tooling::instances::init::init_instance;
use crate::tooling::instances::plugins::handle_plugins;
use crate::tooling::instances::repositories::handle_repository;
use std::path::PathBuf;

pub fn handle_instance(args: InstancesArgs) {
    let instance_dir = args.working_directory.unwrap_or(PathBuf::from("."));
    match args.commands {
        InstancesCommands::Init(args) => init_instance(&instance_dir, args),
        InstancesCommands::Repository(args) => handle_repository(&instance_dir, args),
        InstancesCommands::Plugins(args) => handle_plugins(&instance_dir, args),
    }
}
