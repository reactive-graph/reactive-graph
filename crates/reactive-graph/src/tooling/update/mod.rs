use crate::tooling::update::args::UpdateArgs;
use crate::tooling::update::commands::UpdateCommands;
use crate::tooling::update::release_info::print_release_info;
use crate::tooling::update::release_info::print_release_list;
use crate::tooling::update::release_tag::ReleaseTag;
use anyhow::Result;
use self_update::backends::github::Update;
use self_update::cargo_crate_version;
use self_update::update::ReleaseUpdate;
use self_update::Status;

pub mod args;
pub mod commands;
pub mod release;
pub mod release_tag;

pub mod release_info;

pub const REPO_OWNER: &str = "reactive-graph";
pub const REPO_NAME: &str = "reactive-graph";
pub const TARGET_TRIPLE: &str = env!("VERGEN_CARGO_TARGET_TRIPLE");

pub fn handle_update(args: UpdateArgs) -> Result<()> {
    if let Some(commands) = &args.commands {
        return match commands {
            UpdateCommands::Info(info_args) => print_release_info(&args, info_args),
            UpdateCommands::List(list_args) => print_release_list(list_args),
        };
    };
    let status = execute_update(&args)?;
    println!("Successfully updated to version: {}", status.version());
    Ok(())
}

fn execute_update(args: &UpdateArgs) -> Result<Status> {
    update_from_github(args)?.update().map_err(Into::into)
}

fn update_from_github(args: &UpdateArgs) -> Result<Box<dyn ReleaseUpdate>> {
    let current_bin_name = env!("CARGO_BIN_NAME");
    let release_tag = ReleaseTag::from(args);
    Update::configure()
        .show_download_progress(args.show_download_progress())
        .show_output(args.show_output())
        .no_confirm(args.no_confirm())
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_path_in_archive(&release_tag.bin_path_in_archive(current_bin_name))
        .bin_name(current_bin_name)
        .current_version(cargo_crate_version!())
        .target_version_tag(&release_tag.target_version_tag())
        .build()
        .map_err(Into::into)
}
