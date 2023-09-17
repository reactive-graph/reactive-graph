use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;

use serde_json::Value;

use crate::builder::CommandDefinition;
use crate::builder::CommandDefinitionBuilder;
use crate::component::command::COMPONENT_COMMAND;
use crate::component::CommandProperties::COMMAND_ARGS;
use crate::component::CommandProperties::COMMAND_HELP;
use crate::component::CommandProperties::COMMAND_NAME;
use crate::component::CommandProperties::COMMAND_NAMESPACE;
use crate::component::CommandProperties::COMMAND_RESULT;
use crate::component::COMMAND_PROPERTIES;
use crate::entity::arg::CommandArgs;
use crate::error::CommandArgsError;
use crate::error::CommandExecutionFailed;
use crate::error::NotACommand;
use crate::model_runtime::ActionProperties::TRIGGER;
use crate::model_runtime::LabeledProperties::LABEL;
use crate::model_runtime::COMPONENT_ACTION;
use crate::model_runtime::COMPONENT_LABELED;
use crate::reactive::ReactiveEntity;
use crate::reactive::ReactivePropertyContainer;
use inexor_rgf_graph::ComponentTypeIds;
use inexor_rgf_graph::DataType;
use inexor_rgf_graph::EntityType;
use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_graph::Mutability;
use inexor_rgf_graph::PropertyInstanceGetter;
use inexor_rgf_graph::PropertyInstanceSetter;
use inexor_rgf_graph::PropertyType;
use inexor_rgf_graph::PropertyTypeDefinition;
use inexor_rgf_graph::SocketType;
use inexor_rgf_reactive_api::prelude::*;

pub struct Command(ReactiveEntity);

impl Command {}

impl Command {
    pub fn new(entity: ReactiveEntity) -> Result<Self, NotACommand> {
        if !entity.is_a(&COMPONENT_ACTION) || !entity.is_a(&COMPONENT_COMMAND) {
            return Err(NotACommand);
        }
        Ok(Command(entity))
    }

    pub fn new_unchecked(entity: ReactiveEntity) -> Self {
        Self(entity)
    }

    pub fn builder() -> CommandDefinitionBuilder {
        CommandDefinition::builder()
    }

    pub fn get_entity_type(&self) -> EntityType {
        let components = ComponentTypeIds::new()
            .component(COMPONENT_LABELED.deref())
            .component(COMPONENT_ACTION.deref())
            .component(COMPONENT_COMMAND.deref());
        let properties = COMMAND_PROPERTIES.clone();
        if let Some(args) = self.get(COMMAND_ARGS).and_then(|args| CommandArgs::try_from(args).ok()) {
            for arg in args.to_vec() {
                if !properties.contains_key(&arg.name) {
                    properties.insert(
                        arg.name.clone(),
                        PropertyType::builder()
                            .name(arg.name.clone())
                            .description(arg.help.unwrap_or_default())
                            .data_type(DataType::Any)
                            .socket_type(SocketType::Input)
                            .mutability(Mutability::Mutable)
                            .build(),
                    );
                }
            }
        }
        let entity_type = EntityType::builder()
            .ty(self.0.ty.clone())
            .description(self.0.description.clone())
            .components(components)
            .properties(COMMAND_PROPERTIES.clone())
            .build();
        entity_type
    }

    /// Executes a command
    pub fn execute(&self) -> Result<Option<Value>, CommandExecutionFailed> {
        if !self.0.is_a(&COMPONENT_ACTION) || !self.0.is_a(&COMPONENT_COMMAND) {
            return Err(CommandExecutionFailed::NotACommand);
        }
        self.0.set(TRIGGER.property_name(), Value::Bool(true));
        Ok(self.0.get(COMMAND_RESULT))
    }

    /// Executes a command with the given arguments
    /// Stores the command result in the command result property
    pub fn execute_with_args(&self, args: HashMap<String, Value>) -> Result<Option<Value>, CommandExecutionFailed> {
        if !self.0.is_a(&COMPONENT_ACTION) || !self.0.is_a(&COMPONENT_COMMAND) {
            return Err(CommandExecutionFailed::NotACommand);
        }
        // Check that all given arguments are valid arguments and the properties exists
        match self.args() {
            Ok(command_args) => {
                for property_name in args.keys() {
                    if !command_args.contains(property_name.clone()) {
                        return Err(CommandExecutionFailed::InvalidArgument(property_name.clone()));
                    } else if !self.0.has_property(property_name) {
                        return Err(CommandExecutionFailed::MissingArgumentProperty(property_name.clone()));
                    }
                }
                for command_arg in command_args.to_vec() {
                    if command_arg.required && !args.contains_key(&command_arg.name) {
                        return Err(CommandExecutionFailed::MissingMandatoryArgument(command_arg.name));
                    }
                }
            }
            Err(e) => {
                return Err(CommandExecutionFailed::CommandArgsError(e));
            }
        }
        for (property_name, value) in args {
            if self.0.has_property(&property_name) {
                self.0.set_checked(property_name, value)
            }
        }
        match self.execute() {
            Ok(Some(result)) => {
                self.0.set(COMMAND_RESULT, result.clone());
                Ok(Some(result))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn namespace(&self) -> Option<String> {
        self.0.as_string(COMMAND_NAMESPACE)
    }

    pub fn name(&self) -> Option<String> {
        self.0.as_string(COMMAND_NAME)
    }

    pub fn label(&self) -> Option<String> {
        self.0.as_string(LABEL)
    }

    pub fn args(&self) -> Result<CommandArgs, CommandArgsError> {
        match self.0.get(COMMAND_ARGS) {
            Some(v) => CommandArgs::try_from(v),
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

    // TODO: impl Deref instead
    pub fn get_instance(&self) -> ReactiveEntity {
        self.0.clone()
    }
}

impl Deref for Command {
    type Target = ReactiveEntity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Command {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<ReactiveEntity> for Command {
    type Error = NotACommand;

    fn try_from(entity: ReactiveEntity) -> Result<Self, Self::Error> {
        Command::new(entity)
    }
}

// pub fn command_property_types() -> PropertyTypes {
//     PropertyTypes::new()
//         .property(
//             PropertyType::builder()
//                 .name(LABEL.property_name())
//                 .data_type(DataType::String)
//                 .mutability(Immutable)
//                 .build()
//         )
//         .property(PropertyType::bool(TRIGGER.property_name()))
//         .property(
//             PropertyType::builder()
//                 .name(COMMAND_NAMESPACE.property_name())
//                 .data_type(DataType::String)
//                 .mutability(Immutable)
//                 .build()
//         )
//         .property(
//             PropertyType::builder()
//                 .name(COMMAND_NAME.property_name())
//                 .data_type(DataType::String)
//                 .mutability(Immutable)
//                 .build()
//         )
//         .property(PropertyType::object(COMMAND_ARGS.property_name()))
//         .property(
//             PropertyType::builder()
//                 .name(COMMAND_HELP.property_name())
//                 .data_type(DataType::String)
//                 .mutability(Immutable)
//                 .build()
//         )
//         .property(
//             PropertyType::builder()
//                 .name(COMMAND_RESULT.property_name())
//                 .data_type(DataType::Any)
//                 .build()
//         )
// }

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ops::Deref;

    use serde_json::json;
    use uuid::Uuid;

    use inexor_rgf_graph::ComponentTypeIds;
    use inexor_rgf_graph::PropertyInstances;
    use inexor_rgf_reactive::ReactiveEntity;
    use inexor_rgf_reactive::ReactiveProperties;

    use crate::component::CommandProperties::COMMAND_ARGS;
    use crate::component::CommandProperties::COMMAND_HELP;
    use crate::component::CommandProperties::COMMAND_NAME;
    use crate::component::COMPONENT_COMMAND;
    use crate::entity::Command;
    use crate::error::CommandExecutionFailed;
    use crate::model_runtime::ActionProperties::TRIGGER;
    use crate::model_runtime::COMPONENT_ACTION;
    use crate::reactive::ReactivePropertyContainer;
    use crate::CommandProperties::COMMAND_RESULT;
    use inexor_rgf_graph::EntityTypeId;
    use inexor_rgf_graph::PropertyInstanceGetter;
    use inexor_rgf_graph::PropertyInstanceSetter;
    use inexor_rgf_graph::PropertyTypeDefinition;

    #[test]
    fn test_command() {
        let ty = EntityTypeId::new_from_type("test", "test");
        let reactive_entity = ReactiveEntity::builder().ty(&ty).build();
        assert!(Command::try_from(reactive_entity).is_err());

        let components = ComponentTypeIds::new().component(COMPONENT_ACTION.deref()).component(COMPONENT_COMMAND.deref());
        let properties = PropertyInstances::new()
            .property(&TRIGGER.property_name(), json!(false))
            .property("arg1", json!(0))
            .property("arg2", json!(1))
            .property(COMMAND_RESULT, json!(0))
            .property(COMMAND_NAME, json!("hello_command"))
            .property(
                COMMAND_ARGS,
                json!([
                    {
                        "name": "arg1"
                    },
                    {
                        "name": "arg2",
                        "required": true
                    }
                ]),
            )
            .property(COMMAND_HELP, json!("Help text"));

        let id = Uuid::new_v4();
        let reactive_properties = ReactiveProperties::new_with_id_from_properties(id, properties);
        let reactive_entity = ReactiveEntity::builder()
            .ty(&ty)
            .id(id)
            .components(components)
            .properties(reactive_properties)
            // .component(COMPONENT_ACTION.clone())
            // .component(COMPONENT_COMMAND.clone())
            // .property(&TRIGGER.property_name(), json!(false))
            // .property("arg1", json!(0))
            // .property("arg2", json!(1))
            // .property(COMMAND_RESULT, json!(0))
            // .property(COMMAND_NAME, json!("hello_command"))
            // .property(
            //     COMMAND_ARGS,
            //     json!([
            //         {
            //             "name": "arg1"
            //         },
            //         {
            //             "name": "arg2",
            //             "required": true
            //         }
            //     ]),
            // )
            // .property(COMMAND_HELP, json!("Help text"))
            .build();
        let e1 = reactive_entity.clone();
        let e2 = reactive_entity.clone();
        e1.observe_with_handle(
            &TRIGGER.property_name(),
            move |_| {
                let arg1 = e2.as_u64("arg1").unwrap_or_default();
                let arg2 = e2.as_u64("arg2").unwrap_or_default();
                e2.set(COMMAND_RESULT, json!(arg1 + arg2));
            },
            0,
        );
        let command = Command::new(reactive_entity).expect("Failed to create a command");
        assert_eq!("hello_command", command.name().expect("Failed to get command name"));
        assert_eq!("Help text", command.help().expect("Failed to get help text"));
        assert_eq!(0, e1.as_u64(COMMAND_RESULT).expect("Failed to get initial result"));
        command.execute().expect("Command execution failed");
        assert_eq!(1, e1.as_u64(COMMAND_RESULT).expect("Failed to get changed result"));
        let mut args = HashMap::new();
        args.insert(String::from("arg1"), json!(1));
        args.insert(String::from("arg2"), json!(2));
        let _ = command.execute_with_args(args).expect("Failed to execute command with args");
        assert_eq!(3, e1.as_u64(COMMAND_RESULT).expect("Failed to get changed result"));
        let mut args = HashMap::new();
        args.insert(String::from("arg1"), json!(1));
        args.insert(String::from("arg2"), json!(2));
        args.insert(String::from("arg3"), json!(3));
        assert_eq!(CommandExecutionFailed::InvalidArgument(String::from("arg3")), command.execute_with_args(args).unwrap_err());
        let mut args = HashMap::new();
        args.insert(String::from("arg1"), json!(1));
        assert_eq!(
            CommandExecutionFailed::MissingMandatoryArgument(String::from("arg2")),
            command.execute_with_args(args).unwrap_err()
        );
    }
}
