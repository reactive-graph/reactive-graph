use crate::tooling::install::args::InstallArgs;
use crate::tooling::instances::args::InstancesArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ToolingCommands {
    /// Manage instances.
    Instances(InstancesArgs),

    /// Manage instances.
    Install(InstallArgs),
}
