use crate::shared::completions::args::ShellArgument;
use crate::shared::completions::args::ShellCompletionsActionArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ShellCompletionsCommands {
    /// Prints or installs Shell completions.
    ShellCompletions(ShellCompletionsActionArgs),
}

#[derive(Subcommand, Debug)]
pub enum ShellCompletionsActionCommands {
    /// Prints the shell completions to stdout.
    Print(ShellArgument),

    /// Installs the shell completions.
    #[cfg(target_os = "linux")]
    Install(ShellArgument),
}
