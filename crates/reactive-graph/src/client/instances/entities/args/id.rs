use crate::client::error::CommandError;
use crate::client::error::CommandError::NotFound;
use clap::Args;
use uuid::Uuid;

/// CLI argument which identifies an entity instance by its id.
#[derive(Args, Debug, Clone)]
pub(crate) struct IdArgs {
    /// The id of the entity instance.
    pub id: Uuid,
}

impl IdArgs {
    pub fn not_found(&self) -> CommandError {
        NotFound(format!("The entity instance with the id {} was not found", &self.id))
    }
}

impl From<IdArgs> for Uuid {
    fn from(id: IdArgs) -> Self {
        id.id
    }
}
