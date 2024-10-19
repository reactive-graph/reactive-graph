use crate::tooling::instances::args::InitInstanceArgs;
use crate::tooling::instances::plugins::args::PluginsArgs;
use crate::tooling::instances::repositories::args::RepositoriesArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum InstancesCommands {
    /// Initialize the filesystem structure of a new local instance.
    Init(InitInstanceArgs),
    /// Manage the repositories of a local instance.
    Repository(RepositoriesArgs),
    /// Manage the repositories of a local instance.
    Plugins(PluginsArgs),
}
