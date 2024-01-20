use inexor_rgf_command_model::entity::Command;
use inexor_rgf_command_model::error::NoSuchCommand;
use springtime_di::injectable;

#[injectable]
pub trait CommandManager: Send + Sync {
    /// Returns the command with the given name.
    fn get_command(&self, name: &str) -> Result<Command, NoSuchCommand>;

    /// Returns all commands.
    fn get_commands(&self) -> Vec<Command>;
}
