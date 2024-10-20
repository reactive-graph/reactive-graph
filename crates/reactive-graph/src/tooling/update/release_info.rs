use crate::shared::output_format::RenderTable;
use crate::tooling::update::args::UpdateArgs;
use crate::tooling::update::args::UpdateInfoArgs;
use crate::tooling::update::args::UpdateListArgs;
use crate::tooling::update::release::Release;
use crate::tooling::update::release_tag::ReleaseTag;
use crate::tooling::update::release_tag::RELEASE_TAG_NIGHTLY;
use crate::tooling::update::update_from_github;
use crate::tooling::update::REPO_NAME;
use crate::tooling::update::REPO_OWNER;
use self_update::cargo_crate_version;

pub fn print_release_info(args: &UpdateArgs, info_args: &UpdateInfoArgs) -> anyhow::Result<()> {
    let release_update = update_from_github(args)?;
    let release_info = match ReleaseTag::from(args) {
        ReleaseTag::Nightly => release_update.get_release_version(RELEASE_TAG_NIGHTLY),
        ReleaseTag::Latest => release_update.get_latest_release(),
        ReleaseTag::Current => release_update.get_release_version(cargo_crate_version!()),
        ReleaseTag::Version(version) => release_update.get_release_version(&version),
    }?;
    vec![Release::from(&release_info)].print_table_and_exit(&info_args.output_format)
}

pub fn print_release_list(args: &UpdateListArgs) -> anyhow::Result<()> {
    self_update::backends::github::ReleaseList::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .build()?
        .fetch()?
        .iter()
        .map(Release::from)
        .collect::<Vec<Release>>()
        .print_table_and_exit(&args.output_format);
}
