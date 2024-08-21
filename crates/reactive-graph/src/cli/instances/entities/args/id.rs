use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NotFound;
use clap::Args;
use uuid::Uuid;

/// The entity type.
#[derive(Args, Debug, Clone)]
pub(crate) struct IdArgs {
    /// The id of the entity instance.
    pub id: Uuid,
}

impl IdArgs {
    pub fn not_found(&self) -> CommandError {
        NotFound(format!("EntityInstance {} not found", &self.id))
    }
}

impl From<IdArgs> for Uuid {
    fn from(id: IdArgs) -> Self {
        id.id
    }
}
