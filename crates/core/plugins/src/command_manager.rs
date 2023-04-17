use crate::model_command::entity::Command;
use crate::model_command::error::NoSuchCommand;

pub trait CommandManager: Send + Sync {
    /// Returns the command with the given name.
    fn get_command(&self, name: &str) -> Result<Command, NoSuchCommand>;

    /// Returns all commands.
    fn get_commands(&self) -> Vec<Command>;
}
