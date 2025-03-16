use std::sync::Arc;

use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_command_model::entity::Command;
use reactive_graph_command_model::error::NoSuchCommand;

#[derive(Component)]
pub struct CommandManagerDelegate {
    command_manager: Arc<dyn reactive_graph_command_api::CommandManager + Send + Sync>,
}

impl CommandManagerDelegate {
    pub fn new(command_manager: Arc<dyn reactive_graph_command_api::CommandManager + Send + Sync>) -> Self {
        Self { command_manager }
    }
}

#[component_alias]
impl reactive_graph_plugin_api::CommandManager for CommandManagerDelegate {
    fn get_command(&self, name: &str) -> Result<Command, NoSuchCommand> {
        self.command_manager.get_command(name)
    }

    fn get_commands(&self) -> Vec<Command> {
        self.command_manager.get_commands()
    }
}
