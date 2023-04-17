use async_trait::async_trait;
use serde_json::json;

use crate::api::CommandManager;
use crate::api::CommandRegistrationError;
use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityInstanceManager;
use crate::di::*;
use crate::model::EntityType;
use crate::model::PropertyInstanceGetter;
use crate::model_command::builder::CommandBuilder;
use crate::model_command::component::CommandProperties::COMMAND_NAME;
use crate::model_command::component::COMPONENT_COMMAND;
use crate::model_command::entity::Command;
use crate::model_command::error::NoSuchCommand;

#[component]
pub struct CommandManagerImpl {
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,
}

#[async_trait]
#[provides]
impl CommandManager for CommandManagerImpl {
    fn get_command(&self, name: &str) -> Result<Command, NoSuchCommand> {
        let name = name.into();
        match self
            .entity_instance_manager
            .get_by_component(&COMPONENT_COMMAND)
            .iter()
            .find(|e| e.as_string(COMMAND_NAME).map_or(false, |command_name| command_name == name))
        {
            Some(e) => Command::try_from(e.clone()).map_err(|e| NoSuchCommand::NotACommand(e)),
            None => Err(NoSuchCommand::CommandNotFound(name)),
        }
    }

    fn get_commands(&self) -> Vec<Command> {
        self.entity_instance_manager
            .get_by_component(&COMPONENT_COMMAND)
            .iter()
            .filter_map(|e| Command::try_from(e.clone()).ok())
            .collect()
    }

    fn register_command(&self, command: Command) -> Result<(), CommandRegistrationError> {
        let ty = command.ty();
        if !self.entity_type_manager.has(&ty) {
            return Err(CommandRegistrationError::EntityTypeNotFound(ty));
        }
        let _ = self
            .entity_instance_manager
            .register_reactive_instance(command.get_instance())
            .map_err(|e| CommandRegistrationError::ReactiveEntityInstanceRegistrationError(e));
        Ok(())
    }

    fn register_singleton_command(&self, command: Command, entity_type: EntityType) -> Result<(), CommandRegistrationError> {
        let _ = self
            .entity_type_manager
            .register(entity_type)
            .map_err(|e| CommandRegistrationError::EntityTypeRegistrationError(e))?;
        let _ = self
            .entity_instance_manager
            .register_reactive_instance(command.get_instance())
            .map_err(|e| CommandRegistrationError::ReactiveEntityInstanceRegistrationError(e));
        Ok(())
    }
}

#[async_trait]
impl Lifecycle for CommandManagerImpl {
    async fn init(&self) {
        let entity_instance_manager = self.entity_instance_manager.clone();
        if let Ok((command, entity_type)) = CommandBuilder::new()
            .singleton_from_type("core", "num_commands")
            .help("Number of commands")
            .no_arguments()
            .executor(move |_| json!(entity_instance_manager.get_by_component(&COMPONENT_COMMAND).len()))
            .build_with_type()
        {
            let _ = self.register_singleton_command(command, entity_type);
        }
    }
}
