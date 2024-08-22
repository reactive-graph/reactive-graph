use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NotFound;
use clap::Args;
use uuid::Uuid;

/// CLI argument which identifies a reactive instance by its id.
#[derive(Args, Debug, Clone)]
pub(crate) struct IdArgs {
    /// The id of the reactive instance.
    pub id: Uuid,
}

impl IdArgs {
    pub fn not_found(&self) -> CommandError {
        NotFound(format!("The instance with the id {} was not found", &self.id))
    }
}

impl From<IdArgs> for Uuid {
    fn from(id: IdArgs) -> Self {
        id.id
    }
}
