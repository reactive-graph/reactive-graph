use crate::shared::manpages::args::ManPagesArguments;
use crate::shared::manpages::commands::ManPagesActionCommands;
use crate::shared::manpages::commands::ManPagesCommands;
use crate::shared::manpages::generate::install_man_pages;
use crate::shared::manpages::generate::print_man_pages;
use clap::CommandFactory;
use std::process::exit;

pub mod args;
pub mod commands;
pub mod error;
pub mod generate;

/// Handles the man pages arguments
pub fn handle_man_pages<T: CommandFactory>(args: &ManPagesArguments) {
    if let Some(commands) = &args.commands {
        match commands {
            ManPagesCommands::ManPages(args) => match &args.commands {
                ManPagesActionCommands::Print => {
                    if let Err(e) = print_man_pages(T::command()) {
                        eprintln!("Failed to print man pages: {e}");
                        exit(1);
                    };
                    exit(0);
                }
                ManPagesActionCommands::Install => {
                    if let Err(e) = install_man_pages(T::command()) {
                        eprintln!("Failed to install man pages: {e}");
                        exit(1);
                    }
                    exit(0);
                }
            },
        }
    }
}
