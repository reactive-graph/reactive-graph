use crate::tooling::instances::repositories::args::DeleteRepositoryArgs;
use crate::tooling::instances::repositories::args::InitRepositoryArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum RepositoriesCommands {
    /// Initializes a new local repository in a local instance.
    Init(InitRepositoryArgs),
    /// Removes a local repository.
    Remove(DeleteRepositoryArgs),
}
