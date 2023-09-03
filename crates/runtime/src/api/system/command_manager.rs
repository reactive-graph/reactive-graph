use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::error::reactive::entity::ReactiveEntityRegistrationError;
use crate::error::types::entity::EntityTypeRegistrationError;
use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::model_command::entity::Command;
use crate::model_command::error::NoSuchCommand;

#[derive(Debug)]
pub enum CommandRegistrationError {
    /// The reactive entity instance cannot be created.
    ReactiveEntityRegistrationError(ReactiveEntityRegistrationError),
    EntityTypeNotFound(EntityTypeId),
    EntityTypeRegistrationError(EntityTypeRegistrationError),
}

#[async_trait]
pub trait CommandManager: Send + Sync + Lifecycle {
    /// Returns the command with the given name.
    fn get_command(&self, name: &str) -> Result<Command, NoSuchCommand>;

    /// Returns all commands.
    fn get_commands(&self) -> Vec<Command>;

    /// Registers a new command.
    fn register_command(&self, command: Command) -> Result<(), CommandRegistrationError>;

    /// Registers a new singleton command.
    fn register_singleton_command(&self, command: Command, entity_type: EntityType) -> Result<(), CommandRegistrationError>;
}
