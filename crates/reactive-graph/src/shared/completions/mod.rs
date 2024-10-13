pub mod args;
pub mod commands;
pub mod error;
#[cfg(target_os = "linux")]
pub mod install;

pub mod print;

use crate::shared::completions::args::ShellCompletionsArguments;
use crate::shared::completions::commands::ShellCompletionsActionCommands;
use crate::shared::completions::commands::ShellCompletionsCommands;
use clap::CommandFactory;
#[cfg(target_os = "linux")]
pub use install::install_shell_completions;
pub use print::print_shell_completions;
use std::process::exit;

/// Handles the man pages arguments
pub fn handle_completions<T: CommandFactory>(args: &ShellCompletionsArguments) {
    if let Some(commands) = &args.commands {
        let mut cmd = T::command();
        match commands {
            ShellCompletionsCommands::ShellCompletions(args) => match &args.commands {
                ShellCompletionsActionCommands::Print(args) => {
                    print_shell_completions(args.shell, &mut cmd);
                    exit(0);
                }
                #[cfg(target_os = "linux")]
                ShellCompletionsActionCommands::Install(args) => {
                    if let Err(e) = install_shell_completions(args.shell, args.shell, &mut cmd) {
                        eprintln!("Failed to install man pages: {e}");
                        exit(1);
                    }
                    exit(0);
                }
            },
        }
    }
}
