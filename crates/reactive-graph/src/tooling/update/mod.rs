use crate::tooling::update::args::UpdateArgs;
use crate::tooling::update::commands::UpdateCommands;
use self_update::backends::github::Update;
use self_update::cargo_crate_version;
use self_update::update::Release;
use self_update::update::ReleaseUpdate;
use std::fmt::Display;
use std::fmt::Formatter;
use std::process::exit;

pub mod args;
pub mod commands;

pub const REPO_OWNER: &str = "reactive-graph";
pub const REPO_NAME: &str = "reactive-graph";

pub const RELEASE_TAG_NIGHTLY: &str = "nightly";
pub const RELEASE_TAG_LATEST: &str = "latest";
pub const TARGET_TRIPLE: &str = env!("VERGEN_CARGO_TARGET_TRIPLE");

#[derive(Debug)]
pub enum ReleaseTag {
    Nightly,
    Latest,
    Current,
    Version(String),
}

impl ReleaseTag {
    pub fn bin_path_in_archive(&self, current_bin_name: &str) -> String {
        match self {
            ReleaseTag::Nightly | ReleaseTag::Latest => format!("reactive-graph-{{{{ version }}}}-{{{{ target }}}}/{current_bin_name}"),
            ReleaseTag::Current | ReleaseTag::Version(_) => format!("reactive-graph-v{{{{ version }}}}-{{{{ target }}}}/{current_bin_name}"),
        }
    }

    pub fn target_version_tag(&self) -> String {
        prefix_version(match self {
            ReleaseTag::Nightly | ReleaseTag::Latest => RELEASE_TAG_NIGHTLY,
            ReleaseTag::Current => cargo_crate_version!(),
            ReleaseTag::Version(version) => version,
        })
    }
}

impl From<&UpdateArgs> for ReleaseTag {
    fn from(args: &UpdateArgs) -> Self {
        if args.nightly.unwrap_or_default() {
            return ReleaseTag::Nightly;
        }
        if args.latest.unwrap_or_default() {
            return ReleaseTag::Latest;
        }
        if args.current.unwrap_or_default() {
            return ReleaseTag::Current;
        }
        if let Some(version) = &args.version {
            return ReleaseTag::Version(prefix_version(version));
        }
        ReleaseTag::Latest
    }
}

impl Display for ReleaseTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReleaseTag::Nightly => RELEASE_TAG_NIGHTLY,
                ReleaseTag::Latest => RELEASE_TAG_LATEST,
                ReleaseTag::Current => cargo_crate_version!(),
                ReleaseTag::Version(version) => &version,
            }
        )
    }
}

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
            print_release(release);
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
                for release in releases {
                    print_release(release);
                    println!("---");
                }
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

fn print_release(release: Release) {
    println!("Version: {}", release.version);
    println!("Name: {}", release.name);
    println!("Date: {}", release.date);
    for release_asset in release.assets {
        if release_asset.name.contains(TARGET_TRIPLE) {
            println!("Asset: {}", release_asset.name);
            println!("Download URL: {}", release_asset.download_url);
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

fn prefix_version(version: &str) -> String {
    if version.starts_with("v") || version == RELEASE_TAG_NIGHTLY {
        version.to_string()
    } else {
        format!("v{version}")
    }
}
