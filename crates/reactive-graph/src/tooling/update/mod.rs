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
            UpdateCommands::Info(release_info_args) => print_release_info(&args.repository, &REACTIVE_GRAPH_REPOSITORY, &args.release, release_info_args),
            UpdateCommands::List(release_list_args) => print_release_list(&args.repository, &REACTIVE_GRAPH_REPOSITORY, release_list_args),
        };
    };
    let status = execute_update(&args)?;
    println!("Successfully updated to version: {}", status.version());
    Ok(())
}

fn execute_update(args: &UpdateArgs) -> Result<Status> {
    update_from_github(&args.repository, &REACTIVE_GRAPH_REPOSITORY, &args.release)?
        .update()
        .map_err(Into::into)
}
