use crate::tooling::instances::repositories::commands::RepositoriesCommands;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct RepositoriesArgs {
    #[command(subcommand)]
    pub commands: RepositoriesCommands,
}

#[derive(Parser, Debug)]
pub struct InitRepositoryArgs {
    /// The local name of the repository.
    pub local_name: String,

    /// The remote URL of the repository.
    pub url: Option<String>,
}

impl Default for InitRepositoryArgs {
    fn default() -> Self {
        InitRepositoryArgs {
            local_name: "default".to_string(),
            url: None,
        }
    }
}

#[derive(Parser, Debug)]
pub struct DeleteRepositoryArgs {
    /// The local name of the repository.
    pub local_name: String,

    /// If true, the default repository will be deleted.
    pub force: Option<bool>,
}
