use crate::shared::markdown_help::commands::MarkdownHelpCommands;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct MarkdownHelpArguments {
    #[command(subcommand)]
    pub(crate) commands: Option<MarkdownHelpCommands>,
}
