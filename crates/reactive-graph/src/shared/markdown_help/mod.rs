use args::MarkdownHelpArguments;
use clap::CommandFactory;
use commands::MarkdownHelpCommands;
use std::process::exit;

pub mod args;
pub mod commands;

/// Handles the markdown help arguments
pub fn handle_markdown_help<T: CommandFactory>(args: &MarkdownHelpArguments) {
    if let Some(commands) = &args.commands {
        match commands {
            MarkdownHelpCommands::PrintMarkdownHelp => {
                clap_markdown::print_help_markdown::<T>();
                exit(0);
            }
        }
    }
}
