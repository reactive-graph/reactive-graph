use crate::tooling::instances::args::InitArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum InstancesCommands {
    /// Manage instances.
    Init(InitArgs),
}
