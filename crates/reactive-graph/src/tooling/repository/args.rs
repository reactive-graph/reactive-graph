use crate::tooling::repository::Repository;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct RepositoryArgs {
    /// The repository owner.
    #[clap(short, long)]
    repository_owner: Option<String>,

    /// The repository name.
    #[clap(short, long)]
    repository_name: Option<String>,
}

impl RepositoryArgs {
    pub fn repository_owner(&self, default_repository: &Box<dyn Repository>) -> String {
        self.repository_owner.clone().unwrap_or(default_repository.repository_owner())
    }
    pub fn repository_name(&self, default_repository: &Box<dyn Repository>) -> String {
        self.repository_name.clone().unwrap_or(default_repository.repository_name())
    }
}
