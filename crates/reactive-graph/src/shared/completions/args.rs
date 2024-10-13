use crate::shared::completions::commands::ShellCompletionsActionCommands;
use crate::shared::completions::commands::ShellCompletionsCommands;
use clap::Parser;
use clap_complete::Shell;

#[derive(Parser, Debug)]
pub struct ShellCompletionsArguments {
    #[command(subcommand)]
    pub commands: Option<ShellCompletionsCommands>,
}

#[derive(Parser, Debug)]
pub struct ShellCompletionsActionArgs {
    #[command(subcommand)]
    pub commands: ShellCompletionsActionCommands,
}

#[derive(Parser, Debug)]
pub struct ShellArgument {
    /// The shell.
    #[arg(value_enum)]
    pub shell: Shell,
}
