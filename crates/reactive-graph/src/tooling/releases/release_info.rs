use crate::shared::output_format::RenderTable;
use crate::tooling::releases::args::ReleaseArgs;
use crate::tooling::releases::args::ReleaseInfoArgs;
use crate::tooling::releases::args::ReleaseListArgs;
use crate::tooling::releases::release::Release;
use crate::tooling::releases::release_tag::RELEASE_TAG_NIGHTLY;
use crate::tooling::releases::release_tag::ReleaseTag;
use crate::tooling::releases::update_from_github::update_from_github;
use crate::tooling::repository::Repository;
use crate::tooling::repository::args::RepositoryArgs;
use anyhow::Result;
use self_update::cargo_crate_version;

pub fn print_release_info(
    repository_args: &RepositoryArgs,
    default_repository: &Box<dyn Repository>,
    release_args: &ReleaseArgs,
    release_info_args: &ReleaseInfoArgs,
) -> Result<()> {
    let release_update = update_from_github(repository_args, default_repository, release_args)?;
    let release_info = match ReleaseTag::from(release_args) {
        ReleaseTag::Nightly => release_update.get_release_version(RELEASE_TAG_NIGHTLY),
        ReleaseTag::Latest => release_update.get_latest_release(),
        ReleaseTag::Current => release_update.get_release_version(cargo_crate_version!()),
        ReleaseTag::Version(version) => release_update.get_release_version(&version),
    }?;
    vec![Release::from(&release_info)].print_table_and_exit(&release_info_args.output_format)
}

pub fn print_release_list(repository_args: &RepositoryArgs, default_repository: &Box<dyn Repository>, release_list_args: &ReleaseListArgs) -> Result<()> {
    self_update::backends::github::ReleaseList::configure()
        .repo_owner(&repository_args.repository_owner(default_repository))
        .repo_name(&repository_args.repository_name(default_repository))
        .build()?
        .fetch()?
        .iter()
        .map(Release::from)
        .collect::<Vec<Release>>()
        .print_table_and_exit(&release_list_args.output_format);
}
