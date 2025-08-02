use anyhow::Result;
use self_update::Status;

use crate::tooling::releases::release_info::print_release_info;
use crate::tooling::releases::release_info::print_release_list;
use crate::tooling::releases::update_from_github::update_from_github;
use crate::tooling::update::args::UpdateArgs;
use crate::tooling::update::commands::UpdateCommands;
use crate::tooling::update::repository::REACTIVE_GRAPH_REPOSITORY;

pub mod args;
pub mod commands;
pub mod repository;

pub fn handle_update(args: UpdateArgs) -> Result<()> {
    if let Some(commands) = &args.commands {
        return match commands {
            UpdateCommands::Info(release_info_args) => print_release_info(&args.release, &args.repository, release_info_args, &REACTIVE_GRAPH_REPOSITORY),
            UpdateCommands::List(release_list_args) => print_release_list(&args.repository, release_list_args, &REACTIVE_GRAPH_REPOSITORY),
        };
    };
    let status = execute_update(&args)?;
    println!("Successfully updated to version: {}", status.version());
    Ok(())
}

fn execute_update(args: &UpdateArgs) -> Result<Status> {
    update_from_github(&args.release, &args.repository, &REACTIVE_GRAPH_REPOSITORY)?
        .update()
        .map_err(Into::into)
}
