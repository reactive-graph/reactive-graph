use crate::shared::completions::args::ShellCompletionsArguments;
use crate::shared::info::args::InfoArgs;
#[cfg(target_os = "linux")]
use crate::shared::manpages::args::ManPagesArguments;
use crate::shared::markdown_help::args::MarkdownHelpArguments;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct SharedArguments {
    #[clap(flatten)]
    pub completions: ShellCompletionsArguments,

    #[cfg(target_os = "linux")]
    #[clap(flatten)]
    pub man_pages: ManPagesArguments,

    #[clap(flatten)]
    pub markdown_help: MarkdownHelpArguments,

    #[clap(flatten)]
    pub info: InfoArgs,
}
