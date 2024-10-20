use crate::shared::output_format::OutputFormatArgsOptional;
use crate::tooling::update::commands::UpdateCommands;
use clap::ArgAction;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(disable_version_flag = true)]
pub struct UpdateArgs {
    /// Updates to the nightly release.
    #[clap(group = "release_tag", short = 'n', long, action=ArgAction::SetTrue, conflicts_with_all = ["latest", "current", "version"])]
    pub nightly: Option<bool>,

    /// Updates to the latest release.
    /// Currently, the latest release is the nightly release. This will change in the future.
    #[clap(group = "release_tag", short = 'l', long, action=ArgAction::SetTrue, conflicts_with_all = ["nightly", "current", "version"])]
    pub latest: Option<bool>,

    /// Updates to the current release.
    #[clap(group = "release_tag", short = 'c', long, action=ArgAction::SetTrue, conflicts_with_all = ["nightly", "latest", "version"])]
    pub current: Option<bool>,

    /// Updates to a specific version.
    #[clap(group = "release_tag", short = 'v', long, conflicts_with_all = ["nightly", "latest", "current"])]
    pub version: Option<String>,

    /// Hides the download progress.
    #[clap(long, action=ArgAction::SetTrue)]
    pub hide_download_progress: Option<bool>,

    /// Hides the output.
    #[clap(long, action=ArgAction::SetTrue)]
    pub hide_output: Option<bool>,

    /// Hides the download progress and the output.
    #[clap(short = 'q', long, action=ArgAction::SetTrue)]
    pub quiet: Option<bool>,

    /// Don't ask.
    #[clap(short = 'y', long, action=ArgAction::SetTrue)]
    pub no_confirm: Option<bool>,

    #[command(subcommand)]
    pub commands: Option<UpdateCommands>,
}

impl UpdateArgs {
    pub fn show_download_progress(&self) -> bool {
        !(self.hide_download_progress.unwrap_or_default() || self.quiet.unwrap_or_default())
    }

    pub fn show_output(&self) -> bool {
        !(self.hide_output.unwrap_or_default() || self.quiet.unwrap_or_default())
    }

    pub fn no_confirm(&self) -> bool {
        self.no_confirm.unwrap_or_default()
    }
}

#[derive(Parser, Debug)]
pub struct UpdateInfoArgs {
    #[clap(flatten)]
    pub output_format: OutputFormatArgsOptional,
}

#[derive(Parser, Debug)]
pub struct UpdateListArgs {
    #[clap(flatten)]
    pub output_format: OutputFormatArgsOptional,
}
