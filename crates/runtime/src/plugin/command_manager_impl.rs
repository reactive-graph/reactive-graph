use std::sync::Arc;

use crate::model_command::entity::Command;
use crate::model_command::error::NoSuchCommand;
use crate::plugins::CommandManager;

pub struct CommandManagerImpl {
    command_manager: Arc<dyn crate::api::CommandManager>,
}

impl CommandManagerImpl {
    pub fn new(command_manager: Arc<dyn crate::api::CommandManager>) -> Self {
        Self { command_manager }
    }
}

impl CommandManager for CommandManagerImpl {
    fn get_command(&self, name: &str) -> Result<Command, NoSuchCommand> {
        self.command_manager.get_command(name)
    }

    fn get_commands(&self) -> Vec<Command> {
        self.command_manager.get_commands()
    }
}
