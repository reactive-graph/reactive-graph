use crate::tooling::instances::repositories::args::RepositoriesArgs;
use crate::tooling::instances::repositories::commands::RepositoriesCommands;
use crate::tooling::instances::repositories::init::init_repository;
use crate::tooling::instances::repositories::remove::remove_repository;
use anyhow::Result;
use std::path::PathBuf;

pub mod args;
pub mod commands;
pub mod init;
pub mod remove;

pub fn handle_repository(instance_dir: &PathBuf, args: RepositoriesArgs) -> Result<()> {
    match args.commands {
        RepositoriesCommands::Init(args) => init_repository(instance_dir, args)?,
        RepositoriesCommands::Remove(args) => remove_repository(instance_dir, args)?,
    }
    Ok(())
}
