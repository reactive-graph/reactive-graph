use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::component::command::COMPONENT_COMMAND;
use crate::component::CommandProperties::COMMAND_ARGS;
use crate::component::CommandProperties::COMMAND_HELP;
use crate::component::CommandProperties::COMMAND_NAME;
use crate::component::CommandProperties::COMMAND_NAMESPACE;
use crate::component::CommandProperties::COMMAND_RESULT;
use crate::entity::arg::CommandArgs;
use crate::error::CommandArgsError;
use crate::error::CommandExecutionFailed;
use crate::error::NotACommand;
use crate::model::ComponentContainer;
use crate::model::EntityTypeId;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_runtime::COMPONENT_ACTION;
use crate::model_runtime::PROPERTY_TRIGGER;

pub struct Command(Arc<ReactiveEntityInstance>);

impl Command {
    /// Executes a command
    pub fn execute(&self) -> Result<Option<Value>, CommandExecutionFailed> {
        if !self.0.is_a(&COMPONENT_ACTION) || !self.0.is_a(&COMPONENT_COMMAND) {
            return Err(CommandExecutionFailed::NotACommand);
        }
        self.0.set(PROPERTY_TRIGGER, Value::Bool(true));
        match self.0.get(COMMAND_RESULT) {
            Some(result) => Ok(Some(result)),
            None => Ok(None),
        }
    }

    /// Executes a command with the given arguments
    pub fn execute_with_args(&self, args: HashMap<String, Value>) -> Result<Option<Value>, CommandExecutionFailed> {
        if !self.0.is_a(&COMPONENT_ACTION) || !self.0.is_a(&COMPONENT_COMMAND) {
            return Err(CommandExecutionFailed::NotACommand);
        }
        for (property_name, value) in args {
            self.0.set_checked(property_name, value)
        }
        self.execute()
    }

    pub fn namespace(&self) -> Option<String> {
        self.0.as_string(COMMAND_NAMESPACE)
    }

    pub fn name(&self) -> Option<String> {
        self.0.as_string(COMMAND_NAME)
    }

    pub fn label(&self) -> Option<String> {
        self.0.as_string("label")
    }

    pub fn args(&self) -> Result<CommandArgs, CommandArgsError> {
        match self.0.get(COMMAND_ARGS) {
            Some(v) => CommandArgs::try_from(v).map_err(|e| CommandArgsError::InvalidCommandArgDefinition(e)),
            None => Err(CommandArgsError::CommandArgDefinitionMissing),
        }
    }

    pub fn command(&self) -> Option<clap::Command> {
        let Some(name) = self.name() else {
            return None;
        };
        let Ok(args) = self.args() else {
            return None;
        };
        Some(clap::Command::new(name).args(args.as_args()))
    }

    pub fn help(&self) -> Option<String> {
        self.0.as_string(COMMAND_HELP)
    }

    pub fn ty(&self) -> EntityTypeId {
        self.0.ty.clone()
    }

    pub fn get_instance(&self) -> Arc<ReactiveEntityInstance> {
        self.0.clone()
    }
}

impl TryFrom<Arc<ReactiveEntityInstance>> for Command {
    type Error = NotACommand;

    fn try_from(entity_instance: Arc<ReactiveEntityInstance>) -> Result<Self, Self::Error> {
        if !entity_instance.is_a(&COMPONENT_ACTION) || !entity_instance.is_a(&COMPONENT_COMMAND) {
            return Err(NotACommand);
        }
        Ok(Command(entity_instance))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use crate::component::CommandProperties::COMMAND_ARGS;
    use crate::component::CommandProperties::COMMAND_HELP;
    use crate::component::CommandProperties::COMMAND_NAME;
    use crate::component::COMPONENT_COMMAND;
    use crate::entity::Command;
    use crate::model::EntityTypeId;
    use crate::model::PropertyInstanceGetter;
    use crate::model::PropertyInstanceSetter;
    use crate::model::ReactivePropertyContainer;
    use crate::model_runtime::COMPONENT_ACTION;
    use crate::model_runtime::PROPERTY_TRIGGER;
    use inexor_rgf_core_builder::ReactiveEntityInstanceBuilder;

    #[test]
    fn test_command() {
        let ty = EntityTypeId::new_from_type("test", "test");
        let e = ReactiveEntityInstanceBuilder::new(&ty).build();
        assert!(Command::try_from(e).is_err());

        let e = ReactiveEntityInstanceBuilder::new(&ty)
            .component(COMPONENT_ACTION.clone())
            .component(COMPONENT_COMMAND.clone())
            .property(PROPERTY_TRIGGER, json!(false))
            .property("arg1", json!(0))
            .property("arg2", json!(1))
            .property("result", json!(0))
            .property(COMMAND_NAME, json!("hello_command"))
            .property(
                COMMAND_ARGS,
                json!([
                    {
                        "name": "arg1"
                    },
                    {
                        "name": "arg2"
                    }
                ]),
            )
            .property(COMMAND_HELP, json!("Help text"))
            .build();
        let e1 = e.clone();
        let e2 = e.clone();
        e1.observe_with_handle(
            PROPERTY_TRIGGER,
            move |_| {
                let arg1 = e2.as_u64("arg1").unwrap_or_default();
                let arg2 = e2.as_u64("arg2").unwrap_or_default();
                e2.set("result", json!(arg1 + arg2));
            },
            0,
        );
        let command = Command::try_from(e).expect("Failed to create a command");
        assert_eq!("hello_command", command.name().expect("Failed to get command name"));
        assert_eq!("Help text", command.help().expect("Failed to get help text"));
        assert_eq!(0, e1.as_u64("result").expect("Failed to get initial result"));
        command.execute().expect("Command execution failed");
        assert_eq!(1, e1.as_u64("result").expect("Failed to get changed result"));
        let mut args = HashMap::new();
        args.insert(String::from("arg1"), json!(1));
        args.insert(String::from("arg2"), json!(2));
        let result = command.execute_with_args(args).expect("Failed to execute command with args");
        assert_eq!(3, e1.as_u64("result").expect("Failed to get changed result"));
    }
}
