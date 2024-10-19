use crate::tooling::instances::args::InstancesArgs;
use crate::tooling::update::args::UpdateArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ToolingCommands {
    /// Manage instances.
    Instances(InstancesArgs),

    /// Update the Reactive Graph binary.
    Update(UpdateArgs),
}
