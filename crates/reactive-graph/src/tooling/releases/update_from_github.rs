use crate::tooling::releases::args::ReleaseArgs;
use crate::tooling::releases::release_tag::ReleaseTag;
use crate::tooling::repository::Repository;
use crate::tooling::repository::args::RepositoryArgs;
use self_update::backends::github::Update;
use self_update::cargo_crate_version;
use self_update::update::ReleaseUpdate;

pub fn update_from_github(
    release_args: &ReleaseArgs,
    repository_args: &RepositoryArgs,
    default_repository: &Box<dyn Repository>,
) -> anyhow::Result<Box<dyn ReleaseUpdate>> {
    let current_bin_name = env!("CARGO_BIN_NAME");
    let release_tag = ReleaseTag::from(release_args);
    Update::configure()
        .show_download_progress(release_args.show_download_progress())
        .show_output(release_args.show_output())
        .no_confirm(release_args.no_confirm())
        .repo_owner(&repository_args.repository_owner(&default_repository))
        .repo_name(&repository_args.repository_name(&default_repository))
        .bin_path_in_archive(&release_tag.bin_path_in_archive(current_bin_name))
        .bin_name(current_bin_name)
        .current_version(cargo_crate_version!())
        .target_version_tag(&release_tag.target_version_tag())
        .build()
        .map_err(Into::into)
}
