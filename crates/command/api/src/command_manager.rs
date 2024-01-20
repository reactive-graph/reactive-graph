use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_command_model::error::NoSuchCommand;
use inexor_rgf_command_model::Command;
use inexor_rgf_graph::EntityType;
use inexor_rgf_lifecycle::Lifecycle;

use crate::CommandRegistrationError;

#[injectable]
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
