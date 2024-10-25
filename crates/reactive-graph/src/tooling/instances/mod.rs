pub mod args;
pub mod certificates;
pub mod commands;
pub mod config;
pub mod init;
pub mod logging;
pub mod plugins;
pub mod provisioning;
pub mod repositories;

use crate::tooling::instances::args::InstancesArgs;
use crate::tooling::instances::certificates::handle_generate_certificate;
use crate::tooling::instances::commands::InstancesCommands;
use crate::tooling::instances::config::handle_config;
use crate::tooling::instances::init::init_instance;
use crate::tooling::instances::plugins::handle_plugins;
use crate::tooling::instances::repositories::handle_repository;
use anyhow::Result;
use std::path::PathBuf;

pub fn handle_instance(args: InstancesArgs) -> Result<()> {
    let instance_dir = args.working_directory.unwrap_or(PathBuf::from("."));
    match args.commands {
        InstancesCommands::Config(args) => handle_config(&instance_dir, args)?,
        InstancesCommands::GenerateCertificate(args) => handle_generate_certificate(&instance_dir, args)?,
        InstancesCommands::Init(args) => init_instance(&instance_dir, args)?,
        InstancesCommands::Plugins(args) => handle_plugins(&instance_dir, args)?,
        InstancesCommands::Repository(args) => handle_repository(&instance_dir, args)?,
    };
    Ok(())
}
