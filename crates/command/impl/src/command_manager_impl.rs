use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;

use reactive_graph_command_api::CommandManager;
use reactive_graph_command_api::CommandRegistrationError;
use reactive_graph_command_model::component::COMPONENT_COMMAND;
use reactive_graph_command_model::component::CommandProperties::COMMAND_NAME;
use reactive_graph_command_model::entity::Command;
use reactive_graph_command_model::error::NoSuchCommand;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_type_system_api::EntityTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;

#[derive(Component)]
pub struct CommandManagerImpl {
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    reactive_entity_manager: Arc<dyn ReactiveEntityManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl CommandManager for CommandManagerImpl {
    fn get_command(&self, name: &str) -> Result<Command, NoSuchCommand> {
        let name = name.into();
        match self
            .reactive_entity_manager
            .get_by_component(&COMPONENT_COMMAND)
            .iter()
            .find(|e| e.as_string(COMMAND_NAME).is_some_and(|command_name| command_name == name))
        {
            Some(e) => Command::try_from(e.clone()).map_err(NoSuchCommand::NotACommand),
            None => Err(NoSuchCommand::CommandNotFound(name)),
        }
    }

    fn get_commands(&self) -> Vec<Command> {
        self.reactive_entity_manager
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
            .reactive_entity_manager
            .register_reactive_instance(command.get_instance())
            .map_err(CommandRegistrationError::ReactiveEntityRegistrationError);
        Ok(())
    }

    fn register_singleton_command(&self, command: Command, entity_type: EntityType) -> Result<(), CommandRegistrationError> {
        let _ = self
            .entity_type_manager
            .register(entity_type)
            .map_err(CommandRegistrationError::EntityTypeRegistrationError)?;
        let _ = self
            .reactive_entity_manager
            .register_reactive_instance(command.get_instance())
            .map_err(CommandRegistrationError::ReactiveEntityRegistrationError);
        Ok(())
    }
}

#[async_trait]
impl Lifecycle for CommandManagerImpl {
    async fn init(&self) {
        let reactive_entity_manager = self.reactive_entity_manager.clone();

        let executor = Box::new(move |_: &ReactiveEntity| json!(reactive_entity_manager.get_by_component(&COMPONENT_COMMAND).len()));
        let command = Command::builder()
            .ty(EntityTypeId::new_from_type("core", "num_commands"))
            .help("Number of commands")
            .description("Number of commands")
            .executor(executor)
            .build();
        let entity_type: EntityType = command.get_entity_type();
        let _ = self.register_singleton_command(command, entity_type);

        // if let Ok((command, entity_type)) = Command::new()
        //     .singleton_from_type("core", "num_commands")
        //     .help("Number of commands")
        //     .no_arguments()
        //     .executor(move |_| json!(reactive_entity_manager.get_by_component(&COMPONENT_COMMAND).len()))
        //     .build_with_type()
        // {
        //     let _ = self.register_singleton_command(command, entity_type);
        // }
    }
}
