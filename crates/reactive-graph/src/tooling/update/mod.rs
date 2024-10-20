use crate::tooling::update::args::UpdateArgs;
use crate::tooling::update::commands::UpdateCommands;
use crate::tooling::update::release_table::ReleaseTable;
use crate::tooling::update::release_tag::ReleaseTag;
use crate::tooling::update::release_tag::RELEASE_TAG_NIGHTLY;
use self_update::backends::github::Update;
use self_update::cargo_crate_version;
use self_update::update::ReleaseUpdate;
use std::process::exit;

pub mod args;
pub mod commands;
pub mod release_table;
pub mod release_tag;

pub const REPO_OWNER: &str = "reactive-graph";
pub const REPO_NAME: &str = "reactive-graph";
pub const TARGET_TRIPLE: &str = env!("VERGEN_CARGO_TARGET_TRIPLE");

pub fn handle_update(args: UpdateArgs) {
    if let Some(commands) = &args.commands {
        match commands {
            UpdateCommands::Info => {
                print_release_info(&args);
            }
            UpdateCommands::List => {
                list_releases();
            }
        }
    };
    execute_update(&args);
}

fn execute_update(args: &UpdateArgs) {
    let release_update = update_from_github(args);
    match release_update.update() {
        Ok(status) => {
            println!("Successfully updated to version: {}", status.version());
            exit(0);
        }
        Err(e) => {
            eprintln!("Failed to update: {e}");
            exit(1);
        }
    }
}

fn print_release_info(args: &UpdateArgs) {
    let release_update = update_from_github(args);
    let release_tag = ReleaseTag::from(args);
    let release_info = match release_tag {
        ReleaseTag::Nightly => release_update.get_release_version(RELEASE_TAG_NIGHTLY),
        ReleaseTag::Latest => release_update.get_latest_release(),
        ReleaseTag::Current => release_update.get_release_version(cargo_crate_version!()),
        ReleaseTag::Version(version) => release_update.get_release_version(&version),
    };
    match release_info {
        Ok(release) => {
            ReleaseTable::render_one(&release);
            exit(0);
        }
        Err(e) => {
            eprintln!("Failed to get information about the latest release: {e}");
            exit(1);
        }
    }
}

fn list_releases() {
    let release_list = self_update::backends::github::ReleaseList::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .build();
    match release_list {
        Ok(release_list) => match release_list.fetch() {
            Ok(releases) => {
                ReleaseTable::render(&releases);
                exit(0);
            }
            Err(e) => {
                eprintln!("Failed to fetch information about the releases: {e}");
                exit(1);
            }
        },
        Err(e) => {
            eprintln!("Failed to get information about the releases: {e}");
            exit(1);
        }
    }
}

fn update_from_github(args: &UpdateArgs) -> Box<dyn ReleaseUpdate> {
    let current_bin_name = env!("CARGO_BIN_NAME");
    let current_version = cargo_crate_version!();
    let release_tag = ReleaseTag::from(args);
    match Update::configure()
        .show_download_progress(args.show_download_progress())
        .show_output(args.show_output())
        .no_confirm(args.no_confirm())
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_path_in_archive(&release_tag.bin_path_in_archive(current_bin_name))
        .bin_name(current_bin_name)
        .current_version(current_version)
        .target_version_tag(&release_tag.target_version_tag())
        .build()
    {
        Ok(release_update) => release_update,
        Err(e) => {
            eprintln!("Can't construct release update: {}", e);
            exit(1);
        }
    }
}
