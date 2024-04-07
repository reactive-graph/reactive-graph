use std::ops::Deref;

use serde_json::json;
use serde_json::Value;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::component::CommandProperties::COMMAND_ARGS;
use crate::component::CommandProperties::COMMAND_HELP;
use crate::component::CommandProperties::COMMAND_NAME;
use crate::component::CommandProperties::COMMAND_NAMESPACE;
use crate::component::CommandProperties::COMMAND_RESULT;
use crate::component::COMPONENT_COMMAND;
use crate::entity::Command;
use crate::entity::CommandArgs;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyInstances;
// use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_model_impl::ReactiveProperties;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use reactive_graph_runtime_model::LabeledProperties::LABEL;
use reactive_graph_runtime_model::COMPONENT_ACTION;
use reactive_graph_runtime_model::COMPONENT_LABELED;

pub type CommandExecutor = dyn FnMut(&ReactiveEntity) -> Value + 'static + Send;

#[derive(TypedBuilder)]
#[builder(
    build_method(vis="pub", into=Command),
    builder_method(vis="pub"),
    builder_type(vis="pub", name=CommandDefinitionBuilder),
)]
pub struct CommandDefinition {
    #[builder(setter(into))]
    ty: EntityTypeId,
    #[builder(default=Uuid::new_v4(), setter(into))]
    id: Uuid,
    #[builder(default, setter(strip_option, into))]
    namespace: Option<String>,
    #[builder(default, setter(strip_option, into))]
    name: Option<String>,
    #[builder(default, setter(into))]
    description: String,
    #[builder(default, setter(into))]
    help: String,
    #[builder(default, setter(into))]
    arguments: CommandArgs,
    // #[builder(setter(into))]
    executor: Box<CommandExecutor>,
    // #[builder(setter(into))]
    // scope: String,
    // #[builder(setter(into))]
    // scope: String,
    // .scope("testing")
    // .name("concat")
    // .label("/org/inexor/test/concat")
    // .help("Concatenates two strings")
}

impl From<CommandDefinition> for Command {
    fn from(definition: CommandDefinition) -> Self {
        let handle_id = Uuid::new_v4().as_u128();

        let namespace = definition.namespace.unwrap_or_else(|| definition.ty.namespace());
        let name = definition.name.unwrap_or_else(|| definition.ty.type_name());

        let label = format!("/org/inexor/commands/{namespace}/{name}");

        let components = ComponentTypeIds::new()
            .component(COMPONENT_LABELED.deref())
            .component(COMPONENT_ACTION.deref())
            .component(COMPONENT_COMMAND.deref());
        // components.insert(COMPONENT_ACTION.clone());
        // components.insert(COMPONENT_COMMAND.clone());

        // let properties = PropertyTypes::new()
        //     .property(PropertyType::string("help"));

        //         let label = format!("/org/inexor/commands/{scope}/{name}");
        //         builder.property(COMMAND_NAMESPACE, json!(scope));
        //         builder.property(COMMAND_NAME, json!(name));
        //         builder.component(&COMPONENT_LABELED.clone());

        let properties = PropertyInstances::new()
            .property(LABEL.property_name(), json!(label))
            .property(TRIGGER.property_name(), json!(false))
            .property(COMMAND_NAMESPACE.property_name(), json!(namespace))
            .property(COMMAND_NAME.property_name(), json!(name))
            .property(COMMAND_ARGS.property_name(), definition.arguments.to_value())
            .property(COMMAND_HELP.property_name(), json!(definition.help))
            .property(COMMAND_RESULT, json!(0));

        for arg in definition.arguments.to_vec() {
            if !properties.contains_key(&arg.name) {
                properties.insert(arg.name.clone(), json!(0));
            }
        }

        let reactive_entity = ReactiveEntity::builder()
            .ty(definition.ty)
            .id(definition.id)
            .description(definition.description)
            .components(components)
            .properties(ReactiveProperties::new_with_id_from_properties(definition.id, properties))
            .build();
        let reactive_entity_inner = reactive_entity.clone();
        let mut executor = definition.executor;
        if let Some(property_instance) = reactive_entity.properties.get(&TRIGGER.property_name()) {
            property_instance.stream.read().unwrap().observe_with_handle(
                move |trigger| {
                    if trigger.as_bool().unwrap_or_default() {
                        // let x = executor(&reactive_entity_inner);
                        reactive_entity_inner.set(COMMAND_RESULT, executor(&reactive_entity_inner));
                    }
                },
                handle_id,
            );
        };
        Command::new_unchecked(reactive_entity)
    }
}

#[cfg(test)]
mod tests {
    use crate::Command;
    use crate::CommandArgs;
    use reactive_graph_graph::EntityTypeId;
    use reactive_graph_reactive_model_impl::ReactiveEntity;
    use serde_json::json;

    #[test]
    fn command_builder_test() {
        let args = CommandArgs::new();
        // TODO: fill args
        let executor = Box::new(move |_: &ReactiveEntity| json!("abc"));
        let command = Command::builder()
            .ty(("core", "num_commands"))
            .description("The number of commands")
            .help("Number of commands")
            .arguments(args)
            .executor(executor)
            .build();

        assert_eq!(command.ty(), EntityTypeId::new_from_type("core", "num_commands"));
        // assert!(command.get
    }
}

// pub struct CommandBuilder<S> {
//     ty: Option<EntityTypeId>,
//     builder: Option<ReactiveEntityBuilder>,
//     arguments: CommandArgs,
//     subscriber: Option<Box<CommandExecutor>>,
//     handle_id: Option<u128>,
//     state: PhantomData<S>,
// }
//
// pub mod command_builder_state {
//     pub enum EntityType {}
//     pub enum Scope {}
//     pub enum Name {}
//     pub enum Label {}
//     pub enum Help {}
//     pub enum Components {}
//     pub enum Arguments {}
//     pub enum Properties {}
//     pub enum Executor {}
//     pub enum Finish {}
// }
//
// impl Default for CommandBuilder<command_builder_state::EntityType> {
//     fn default() -> Self {
//         Self::new()
//     }
// }
//
// impl CommandBuilder<command_builder_state::EntityType> {
//     pub fn new() -> CommandBuilder<command_builder_state::EntityType> {
//         Self {
//             ty: None,
//             builder: None,
//             arguments: CommandArgs::new(),
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
//
//     pub fn ty(self, ty: &EntityTypeId) -> CommandBuilder<command_builder_state::Scope> {
//         let mut builder = ReactiveEntityBuilder::new(ty.clone());
//         builder.component(&COMPONENT_ACTION.clone());
//         builder.component(&COMPONENT_COMMAND.clone());
//         builder.property(TRIGGER.property_name(), json!(false));
//         builder.property(COMMAND_RESULT, json!(0));
//         CommandBuilder {
//             ty: Some(ty.clone()),
//             builder: Some(builder),
//             arguments: CommandArgs::new(),
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
//
//     /// Uses the type information to build a command.
//     /// Useful for entity types with exactly one instance.
//     pub fn singleton(self, ty: &EntityTypeId) -> CommandBuilder<command_builder_state::Help> {
//         let command = ReactiveEntity::builder()
//             .ty(ty)
//             .components(vec![
//                 &COMPONENT_ACTION,
//                 &COMPONENT_COMMAND
//             ])
//             // .properties(vec![
//             //     TRIGGER.property_name(), json!(false)
//             // ])
//             .build();
//
//         let entity_instance = ReactiveEntity::builder()
//             .ty(ty)
//             .components(
//                 Components::new()
//
//             )
//             .build();
//         let mut builder = ReactiveEntityBuilder::new(ty.clone());
//         builder.component(&COMPONENT_ACTION.clone());
//         builder.component(&COMPONENT_COMMAND.clone());
//         builder.property(TRIGGER.property_name(), json!(false));
//         builder.property(COMMAND_RESULT, json!(0));
//         let scope = ty.namespace();
//         let name = ty.type_name();
//         let label = format!("/org/inexor/commands/{scope}/{name}");
//         builder.property(COMMAND_NAMESPACE, json!(scope));
//         builder.property(COMMAND_NAME, json!(name));
//         builder.component(&COMPONENT_LABELED.clone());
//         builder.property("label", json!(label));
//         CommandBuilder {
//             ty: Some(ty.clone()),
//             builder: Some(builder),
//             arguments: CommandArgs::new(),
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
//
//     pub fn singleton_from_type<S1: Into<String>, S2: Into<String>>(self, namespace: S1, type_name: S2) -> CommandBuilder<command_builder_state::Help> {
//         let ty = EntityTypeId::new_from_type(namespace.into(), type_name.into());
//         self.singleton(&ty)
//     }
// }
//
// impl CommandBuilder<command_builder_state::Scope> {
//     pub fn scope<S: Into<String>>(mut self, scope: S) -> CommandBuilder<command_builder_state::Name> {
//         if let Some(builder) = self.builder.as_mut() {
//             builder.property(COMMAND_NAMESPACE, json!(scope.into()));
//         }
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
//
//     pub fn scope_and_name<S1: Into<String>, S2: Into<String>>(mut self, scope: S1, name: S2) -> CommandBuilder<command_builder_state::Help> {
//         if let Some(builder) = self.builder.as_mut() {
//             let scope = scope.into();
//             let name = name.into();
//             let label = format!("/org/inexor/commands/{scope}/{name}");
//             builder.property(COMMAND_NAMESPACE, json!(scope));
//             builder.property(COMMAND_NAME, json!(name));
//             builder.component(&COMPONENT_LABELED.clone());
//             builder.property("label", json!(label));
//         }
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
// }
//
// impl CommandBuilder<command_builder_state::Name> {
//     pub fn name<S: Into<String>>(mut self, name: S) -> CommandBuilder<command_builder_state::Label> {
//         if let Some(builder) = self.builder.as_mut() {
//             builder.property(COMMAND_NAME, json!(name.into()));
//         }
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
// }
//
// impl CommandBuilder<command_builder_state::Label> {
//     pub fn label<S: Into<String>>(mut self, label: S) -> CommandBuilder<command_builder_state::Help> {
//         if let Some(builder) = self.builder.as_mut() {
//             builder.component(&COMPONENT_LABELED.clone());
//             builder.property("label", json!(label.into()));
//         }
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
//
//     pub fn no_label(self) -> CommandBuilder<command_builder_state::Help> {
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
// }
//
// impl CommandBuilder<command_builder_state::Help> {
//     pub fn help<S: Into<String>>(mut self, help: S) -> CommandBuilder<command_builder_state::Components> {
//         if let Some(builder) = self.builder.as_mut() {
//             builder.property(COMMAND_HELP, json!(help.into()));
//         }
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
//
//     pub fn no_help(self) -> CommandBuilder<command_builder_state::Components> {
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
// }
//
// impl CommandBuilder<command_builder_state::Components> {
//     pub fn component(mut self, ty: &ComponentTypeId) -> CommandBuilder<command_builder_state::Components> {
//         if let Some(builder) = self.builder.as_mut() {
//             builder.component(ty.clone());
//         }
//         self
//     }
//
//     pub fn component_from_type<S1: Into<String>, S2: Into<String>>(
//         mut self,
//         namespace: S1,
//         type_name: S2,
//     ) -> CommandBuilder<command_builder_state::Components> {
//         if let Some(builder) = self.builder.as_mut() {
//             let ty = ComponentTypeId::new_from_type(namespace.into(), type_name.into());
//             builder.component(&ty);
//         }
//         self
//     }
//
//     pub fn arguments(self) -> CommandBuilder<command_builder_state::Arguments> {
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
//
//     pub fn no_arguments(self) -> CommandBuilder<command_builder_state::Executor> {
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
// }
//
// impl CommandBuilder<command_builder_state::Arguments> {
//     pub fn argument<A: Into<CommandArg>>(mut self, arg: A, value: Value) -> CommandBuilder<command_builder_state::Arguments> {
//         let arg = arg.into();
//         if let Some(builder) = self.builder.as_mut() {
//             builder.property(arg.name.clone(), value);
//             self.arguments.push(arg);
//         }
//         self
//     }
//
//     fn create_arguments_property(&mut self) {
//         if let Some(builder) = self.builder.as_mut() {
//             builder.property(COMMAND_ARGS, self.arguments.to_value());
//         }
//     }
//
//     pub fn properties(mut self) -> CommandBuilder<command_builder_state::Properties> {
//         self.create_arguments_property();
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
//
//     pub fn no_properties(mut self) -> CommandBuilder<command_builder_state::Executor> {
//         self.create_arguments_property();
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
// }
//
// impl CommandBuilder<command_builder_state::Properties> {
//     pub fn property<S: Into<String>>(mut self, property_name: S, value: Value) -> CommandBuilder<command_builder_state::Properties> {
//         if let Some(builder) = self.builder.as_mut() {
//             builder.property(property_name.into(), value);
//         }
//         self
//     }
//
//     pub fn no_more_properties(self) -> CommandBuilder<command_builder_state::Executor> {
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: None,
//             handle_id: None,
//             state: PhantomData,
//         }
//     }
// }
//
// impl CommandBuilder<command_builder_state::Executor> {
//     pub fn executor<F>(self, subscriber: F) -> CommandBuilder<command_builder_state::Finish>
//     where
//         F: FnMut(&ReactiveEntity) -> Value + 'static + Send,
//     {
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: Some(Box::new(subscriber)),
//             handle_id: self.handle_id,
//             state: PhantomData,
//         }
//     }
//
//     pub fn executor_with_handle<F>(self, subscriber: F, handle_id: Option<u128>) -> CommandBuilder<command_builder_state::Finish>
//     where
//         F: FnMut(&ReactiveEntity) -> Value + 'static + Send,
//     {
//         CommandBuilder {
//             ty: self.ty,
//             builder: self.builder,
//             arguments: self.arguments,
//             subscriber: Some(Box::new(subscriber)),
//             handle_id,
//             state: PhantomData,
//         }
//     }
// }
//
// impl CommandBuilder<command_builder_state::Finish> {
//     pub fn id(mut self, id: Uuid) -> CommandBuilder<command_builder_state::Finish> {
//         if let Some(builder) = self.builder.as_mut() {
//             builder.id(id);
//         };
//         self
//     }
//
//     pub fn build(self) -> Result<Command, CommandBuilderError> {
//         let Some(builder) = self.builder else {
//             return Err(CommandBuilderError::NotACommand);
//         };
//         let Some(mut subscriber) = self.subscriber else {
//             return Err(CommandBuilderError::MissingExecutor);
//         };
//
//         let entity_instance = builder.build();
//         let e = entity_instance.clone();
//         let Some(property_instance) = e.properties.get(&TRIGGER.property_name()) else {
//             return Err(CommandBuilderError::MissingTrigger);
//         };
//
//         let entity_instance_inner = entity_instance.clone();
//         let handle_id = self.handle_id.unwrap_or(Uuid::new_v4().as_u128());
//         property_instance.stream.read().unwrap().observe_with_handle(
//             move |trigger| {
//                 if trigger.as_bool().unwrap_or_default() {
//                     entity_instance_inner.set(COMMAND_RESULT, subscriber(&entity_instance_inner));
//                 }
//             },
//             handle_id,
//         );
//         Command::try_from(entity_instance).map_err(|_| CommandBuilderError::NotACommand)
//     }
//
//     pub fn build_with_type(self) -> Result<(Command, EntityType), CommandBuilderError> {
//         let Some(builder) = self.builder else {
//             return Err(CommandBuilderError::NotACommand);
//         };
//         let Some(mut subscriber) = self.subscriber else {
//             return Err(CommandBuilderError::MissingExecutor);
//         };
//
//         let entity_instance = builder.build();
//         let e = entity_instance.clone();
//         let Some(property_instance) = e.properties.get(&TRIGGER.property_name()) else {
//             return Err(CommandBuilderError::MissingTrigger);
//         };
//
//         let entity_instance_inner = entity_instance.clone();
//         let handle_id = self.handle_id.unwrap_or(Uuid::new_v4().as_u128());
//         property_instance.stream.read().unwrap().observe_with_handle(
//             move |trigger| {
//                 if trigger.as_bool().unwrap_or_default() {
//                     entity_instance_inner.set(COMMAND_RESULT, subscriber(&entity_instance_inner));
//                 }
//             },
//             handle_id,
//         );
//         let entity_type = EntityType::builder()
//             .ty(self.ty.unwrap())
//             .description(entity_instance.as_string(COMMAND_HELP).unwrap_or_default())
//             .components(ComponentTypeIds::from(&entity_instance.components))
//             .properties(self.arguments.to_property_types())
//             .build();
//         Command::try_from(entity_instance)
//             .map_err(|_| CommandBuilderError::NotACommand)
//             .map(|command| (command, entity_type))
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;
//
//     use serde_json::json;
//
//     use crate::builder::CommandBuilder;
//     use crate::entity::CommandArg;
//     use reactive_graph_graph::ComponentTypeId;
//     use reactive_graph_graph::EntityTypeId;
//     use reactive_graph_graph::PropertyInstanceGetter;
//     use reactive_graph_reactive_model_api::ReactivePropertyContainer;
//
//     #[test]
//     fn test_builder() {
//         let command = CommandBuilder::new()
//             .ty(&EntityTypeId::new_from_type("testing", "concat"))
//             .scope("testing")
//             .name("concat")
//             .label("/org/inexor/test/concat")
//             .help("Concatenates two strings")
//             // Additional components
//             .component(&ComponentTypeId::new_from_type("test", "test"))
//             // Arguments
//             .arguments()
//             .argument(
//                 CommandArg::new("argument1")
//                     .short('a')
//                     .long("argument1")
//                     .help("The first argument")
//                     .required(true),
//                 json!(""),
//             )
//             .argument(CommandArg::new("argument2").short('b').long("argument2").help("The second argument"), json!(""))
//             // Additional properties
//             .properties()
//             .property("something", json!(""))
//             .no_more_properties()
//             .executor(|e| {
//                 let mut result = String::new();
//                 if let Some(argument1) = e.as_string("argument1") {
//                     result.push_str(&argument1);
//                 }
//                 if let Some(argument2) = e.as_string("argument2") {
//                     result.push_str(&argument2);
//                 }
//                 json!(result)
//             })
//             .build()
//             .expect("Failed to create command");
//         assert_eq!("testing", command.namespace().expect("No command namespace"));
//         assert_eq!("concat", command.name().expect("No command name"));
//         assert_eq!("Concatenates two strings", command.help().expect("No help text"));
//
//         assert!(command.get_instance().has_property("argument1"));
//         assert!(command.get_instance().has_property("argument2"));
//         assert!(command.get_instance().has_property("something"));
//
//         let args = command.args().expect("No command args");
//         assert_eq!(2, args.len());
//         assert!(args.contains("argument1"));
//         assert!(args.contains("argument2"));
//         assert!(!args.contains("something"));
//
//         let mut exec_args = HashMap::new();
//         exec_args.insert(String::from("argument1"), json!("Hello, "));
//         exec_args.insert(String::from("argument2"), json!("World"));
//         let command_result = command
//             .execute_with_args(exec_args)
//             .expect("Command execution failed")
//             .expect("No return value")
//             .as_str()
//             .expect("Failed to extract command result string")
//             .to_string();
//         assert_eq!("Hello, World", command_result);
//     }
//
//     #[test]
//     fn test_builder_scope_and_name() {
//         let command = CommandBuilder::new()
//             .ty(&EntityTypeId::new_from_type("testing", "test"))
//             .scope_and_name("testing", "test")
//             .help("A test command")
//             .no_arguments()
//             .executor(|_| json!(""))
//             .build()
//             .expect("Failed to create command");
//         assert_eq!("testing", command.namespace().expect("No command namespace"));
//         assert_eq!("test", command.name().expect("No command name"));
//         // Automatically generated label
//         assert_eq!("/org/inexor/commands/testing/test", command.label().expect("No label"));
//         assert_eq!("A test command", command.help().expect("No help text"));
//     }
//
//     #[test]
//     fn test_builder_singleton() {
//         // Singleton Command
//         // command scope = entity type namespace
//         // command name = entity type name
//         let command = CommandBuilder::new()
//             .singleton_from_type("testing", "add")
//             .help("Adds two numbers")
//             .arguments()
//             .argument(CommandArg::new("lhs").short('l').long("lhs").help("The left hand side argument").required(true), json!(0))
//             .argument(
//                 CommandArg::new("rhs")
//                     .short('r')
//                     .long("rhs")
//                     .help("The right hand side argument")
//                     .required(true),
//                 json!(0),
//             )
//             .no_properties()
//             .executor(|e| {
//                 let mut result = 0;
//                 if let (Some(lhs), Some(rhs)) = (e.as_i64("lhs"), e.as_i64("rhs")) {
//                     result = lhs + rhs;
//                 }
//                 json!(result)
//             })
//             .build()
//             .expect("Failed to create command");
//         assert_eq!("testing", command.namespace().expect("No command namespace"));
//         assert_eq!("add", command.name().expect("No command name"));
//         // Automatically generated label
//         assert_eq!("/org/inexor/commands/testing/add", command.label().expect("No label"));
//         assert_eq!("Adds two numbers", command.help().expect("No help text"));
//         let mut exec_args = HashMap::new();
//         exec_args.insert(String::from("lhs"), json!(1));
//         exec_args.insert(String::from("rhs"), json!(2));
//         let command_result = command
//             .execute_with_args(exec_args)
//             .expect("Command execution failed")
//             .expect("No return value");
//         assert_eq!(3, command_result.as_i64().unwrap());
//     }
// }
