use crate::tooling::instances::plugins::install::args::InstallPluginsFromRepositoryArgs;
use crate::tooling::instances::plugins::install::commands::InstallPluginsFromRepositoryCommands;
use crate::tooling::instances::plugins::install::repositories::std::PLUGINS_REPOSITORY_STD;
use crate::tooling::releases::release_info::print_release_info;
use crate::tooling::releases::release_info::print_release_list;
use crate::tooling::releases::release_tag::RELEASE_TAG_NIGHTLY;
use crate::tooling::releases::release_tag::ReleaseTag;
use crate::tooling::releases::update_from_github::update_from_github;
use anyhow::Result;
use anyhow::anyhow;
use self_update::cargo_crate_version;
use std::fs::File;
use std::path::Path;

pub mod args;
pub mod commands;
pub mod repositories;

pub fn install_plugin(instance_dir: &Path, install_args: InstallPluginsFromRepositoryArgs) -> Result<()> {
    if let Some(commands) = &install_args.commands {
        return match commands {
            InstallPluginsFromRepositoryCommands::Info(release_info_args) => {
                print_release_info(&install_args.release, &install_args.repository, release_info_args, &PLUGINS_REPOSITORY_STD)
            }
            InstallPluginsFromRepositoryCommands::List(release_list_args) => {
                print_release_list(&install_args.repository, release_list_args, &PLUGINS_REPOSITORY_STD)
            }
        };
    };
    execute_install(instance_dir, &install_args)?;
    println!(
        "Successfully installed plugins from repository {}/{}",
        install_args.repository.repository_owner(&PLUGINS_REPOSITORY_STD),
        install_args.repository.repository_name(&PLUGINS_REPOSITORY_STD),
    );
    Ok(())
}

fn execute_install(instance_dir: &Path, install_args: &InstallPluginsFromRepositoryArgs) -> Result<()> {
    let release_args = &install_args.release;
    let release_update = update_from_github(&release_args, &install_args.repository, &PLUGINS_REPOSITORY_STD)?;
    let release_tag = ReleaseTag::from(release_args);
    if release_args.show_output() {
        println!("Release tag: {}", release_tag);
    }
    let release_info = match release_tag {
        ReleaseTag::Nightly => release_update.get_release_version(RELEASE_TAG_NIGHTLY),
        ReleaseTag::Latest => release_update.get_latest_release(),
        ReleaseTag::Current => release_update.get_release_version(cargo_crate_version!()),
        ReleaseTag::Version(version) => release_update.get_release_version(&version),
    }?;
    if release_args.show_output() {
        println!(
            "Release:\n  Name:    {}\n  Version: {}\n  Date:    {}",
            release_info.name, release_info.version, release_info.date
        );
    }
    let release_asset = release_info.asset_for(&self_update::get_target(), None).ok_or(anyhow!(
        "Failed to get release asset for release {} (v{}, {})",
        release_info.name,
        release_info.version,
        release_info.date
    ))?;
    if release_args.show_output() {
        println!("Release asset:\n  Name: {}\n  URL:  {}", release_asset.name, release_asset.download_url);
    }
    let deploy_path = instance_dir.join("plugins").join("deploy").canonicalize()?;
    let install_path = deploy_path.join(&release_asset.name);
    if release_args.show_output() {
        println!("Install Path: {}", install_path.display());
    }
    let install_file = File::create(&install_path)?;
    if release_args.show_output() {
        println!("Downloading {} from {} to {}", &release_asset.name, &release_asset.download_url, &deploy_path.display());
    }
    self_update::Download::from_url(&release_asset.download_url)
        .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
        .show_progress(release_args.show_download_progress())
        .download_to(&install_file)?;

    Ok(())
}
