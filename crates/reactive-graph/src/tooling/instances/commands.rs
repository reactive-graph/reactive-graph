use crate::tooling::instances::certificates::args::GenerateCertificateArgs;
use crate::tooling::instances::config::args::ConfigInstanceArgs;
use crate::tooling::instances::init::args::InitInstanceArgs;
use crate::tooling::instances::plugins::args::PluginsArgs;
use crate::tooling::instances::repositories::args::RepositoriesArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum InstancesCommands {
    /// Configures a local instance,
    Config(ConfigInstanceArgs),
    /// Generates certificate of a local instance.
    GenerateCertificate(GenerateCertificateArgs),
    /// Initialize the filesystem structure of a new local instance.
    Init(InitInstanceArgs),
    /// Manage the plugins of a local instance.
    Plugins(PluginsArgs),
    /// Manage the repositories of a local instance.
    Repository(RepositoriesArgs),
}
