use crate::shared::info::args::InfoCommandArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum InfoCommands {
    /// Prints info about this binary.
    Info(InfoCommandArgs),
}
