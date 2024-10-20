use crate::tooling::update::args::UpdateInfoArgs;
use crate::tooling::update::args::UpdateListArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum UpdateCommands {
    /// Shows information about the selected release.
    Info(UpdateInfoArgs),
    /// Lists the releases.
    List(UpdateListArgs),
}
