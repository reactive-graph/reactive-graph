//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::command::NumCommands`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const NUM_COMMANDS_NAMESPACE: &str = "reactive_graph::command::NumCommands";

/// The [type identifier]() of EntityType `NumCommands`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::command::NumCommands`
pub static NUM_COMMANDS: std::sync::LazyLock<reactive_graph_graph::EntityTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(NUM_COMMANDS_NAMESPACE).unwrap());

/// The properties of EntityType `NumCommands`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum NumCommandsProperties {
    
    /// ### Property `args`
    ///
    /// The command arguments
    ///
    /// Data Type: `Array`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    ARGS,
    
    /// ### Property `cmd_ignore`
    ///
    /// blah
    ///
    /// Data Type: `Any`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    CMD_IGNORE,
    
    /// ### Property `cmd_result`
    ///
    /// The result of the command
    ///
    /// Data Type: `Any`
    ///
    /// Socket Type: `Output`
    ///
    /// Mutability: `Immutable`
    ///
    CMD_RESULT,
    
    /// ### Property `command`
    ///
    /// The command name
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    COMMAND,
    
    /// ### Property `help`
    ///
    /// Help text which explains the command.
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    HELP,
    
    /// ### Property `namespace`
    ///
    /// The command namespace
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    NAMESPACE,
    
    /// ### Property `test`
    ///
    /// Data Type: `Bool`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    TEST,
}
impl NumCommandsProperties {
    pub fn len() -> usize {
        7usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types.push(NumCommandsProperties::ARGS);
        property_types.push(NumCommandsProperties::CMD_IGNORE);
        property_types.push(NumCommandsProperties::CMD_RESULT);
        property_types.push(NumCommandsProperties::COMMAND);
        property_types.push(NumCommandsProperties::HELP);
        property_types.push(NumCommandsProperties::NAMESPACE);
        property_types.push(NumCommandsProperties::TEST);
        property_types
    }
}

impl AsRef<str> for NumCommandsProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {
            NumCommandsProperties::ARGS => "args",
            NumCommandsProperties::CMD_IGNORE => "cmd_ignore",
            NumCommandsProperties::CMD_RESULT => "cmd_result",
            NumCommandsProperties::COMMAND => "command",
            NumCommandsProperties::HELP => "help",
            NumCommandsProperties::NAMESPACE => "namespace",
            NumCommandsProperties::TEST => "test",
        }
    }
}

impl From<NumCommandsProperties> for &'static str {
    #[inline]
    fn from(properties: NumCommandsProperties) -> &'static str {
        match properties {
            NumCommandsProperties::ARGS => "args",
            NumCommandsProperties::CMD_IGNORE => "cmd_ignore",
            NumCommandsProperties::CMD_RESULT => "cmd_result",
            NumCommandsProperties::COMMAND => "command",
            NumCommandsProperties::HELP => "help",
            NumCommandsProperties::NAMESPACE => "namespace",
            NumCommandsProperties::TEST => "test",
        }
    }
}

impl From<NumCommandsProperties> for String {
    #[inline]
    fn from(properties: NumCommandsProperties) -> String {
        match properties {
            NumCommandsProperties::ARGS => "args".to_owned(),
            NumCommandsProperties::CMD_IGNORE => "cmd_ignore".to_owned(),
            NumCommandsProperties::CMD_RESULT => "cmd_result".to_owned(),
            NumCommandsProperties::COMMAND => "command".to_owned(),
            NumCommandsProperties::HELP => "help".to_owned(),
            NumCommandsProperties::NAMESPACE => "namespace".to_owned(),
            NumCommandsProperties::TEST => "test".to_owned(),
        }
    }
}

impl From<NumCommandsProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(properties: NumCommandsProperties) -> reactive_graph_graph::PropertyType {
        match properties {
            NumCommandsProperties::ARGS => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "args",
                    "The command arguments",
                    reactive_graph_graph::DataType::Array,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Immutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            NumCommandsProperties::CMD_IGNORE => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "cmd_ignore",
                    "blah",
                    reactive_graph_graph::DataType::Any,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            NumCommandsProperties::CMD_RESULT => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "cmd_result",
                    "The result of the command",
                    reactive_graph_graph::DataType::Any,
                    reactive_graph_graph::SocketType::Output,
                    reactive_graph_graph::Mutability::Immutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            NumCommandsProperties::COMMAND => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "command",
                    "The command name",
                    reactive_graph_graph::DataType::String,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Immutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            NumCommandsProperties::HELP => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "help",
                    "Help text which explains the command.",
                    reactive_graph_graph::DataType::String,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Immutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            NumCommandsProperties::NAMESPACE => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "namespace",
                    "The command namespace",
                    reactive_graph_graph::DataType::String,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Immutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            NumCommandsProperties::TEST => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "test",
                    "",
                    reactive_graph_graph::DataType::Bool,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
        }
    }
}

pub struct NumCommandsPropertiesIterator(Option<NumCommandsProperties>);

impl NumCommandsProperties {
    pub fn into_iter() -> NumCommandsPropertiesIterator {
        NumCommandsPropertiesIterator(None)
    }
}

impl Iterator for NumCommandsPropertiesIterator {
    type Item = NumCommandsProperties;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(NumCommandsProperties::ARGS),
            Some(NumCommandsProperties::ARGS) => Some(NumCommandsProperties::CMD_IGNORE),
            Some(NumCommandsProperties::CMD_IGNORE) => {
                Some(NumCommandsProperties::CMD_RESULT)
            }
            Some(NumCommandsProperties::CMD_RESULT) => {
                Some(NumCommandsProperties::COMMAND)
            }
            Some(NumCommandsProperties::COMMAND) => Some(NumCommandsProperties::HELP),
            Some(NumCommandsProperties::HELP) => Some(NumCommandsProperties::NAMESPACE),
            Some(NumCommandsProperties::NAMESPACE) => Some(NumCommandsProperties::TEST),
            Some(NumCommandsProperties::TEST) => None,
        };
        self.0.clone()
    }
}

impl IntoIterator for NumCommandsProperties {
    type Item = NumCommandsProperties;
    type IntoIter = NumCommandsPropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        NumCommandsPropertiesIterator(None)
    }
}

impl core::fmt::Display for NumCommandsProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {
            NumCommandsProperties::ARGS => core::fmt::Display::fmt("args", f),
            NumCommandsProperties::CMD_IGNORE => core::fmt::Display::fmt("cmd_ignore", f),
            NumCommandsProperties::CMD_RESULT => core::fmt::Display::fmt("cmd_result", f),
            NumCommandsProperties::COMMAND => core::fmt::Display::fmt("command", f),
            NumCommandsProperties::HELP => core::fmt::Display::fmt("help", f),
            NumCommandsProperties::NAMESPACE => core::fmt::Display::fmt("namespace", f),
            NumCommandsProperties::TEST => core::fmt::Display::fmt("test", f),
        }
    }
}

/// ## Components
///
/// | Component                          | Description                                                                             | Properties                                                                                                                      |
/// |------------------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------|
/// | `reactive_graph::command::Command` | A command which can be executed. The command has a name and can have command arguments. | <ul compact><li>`cmd_ignore`</li><li>`cmd_result`</li><li>`namespace`</li><li>`help`</li><li>`args`</li><li>`command`</li></ul> |
///
pub static NUM_COMMANDS_COMPONENTS: std::sync::LazyLock<
    reactive_graph_graph::ComponentTypeIds,
> = std::sync::LazyLock::new(|| {
    reactive_graph_graph::ComponentTypeIds::new()
        .component(
            std::ops::Deref::deref(&crate::reactive_graph::command::command::COMMAND),
        )
});

///
pub static NUM_COMMANDS_EXTENSIONS: std::sync::LazyLock<
    reactive_graph_graph::Extensions,
> = std::sync::LazyLock::new(|| { reactive_graph_graph::Extensions::new() });

pub static NUM_COMMANDS_TYPE: std::sync::LazyLock<reactive_graph_graph::EntityType> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::EntityType::builder()
        .ty(core::ops::Deref::deref(&NUM_COMMANDS))
        .description("The number of commands")
        .components(NUM_COMMANDS_COMPONENTS.clone())
        .properties(NumCommandsProperties::property_types())
        .extensions(NUM_COMMANDS_EXTENSIONS.clone())
        .build()
});

/// # EntityType `NumCommands`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::command::NumCommands`
///
/// ## Description
///
/// The number of commands
///
/// ## Components
///
/// | Component                          | Description                                                                             | Properties                                                                                                                      |
/// |------------------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------|
/// | `reactive_graph::command::Command` | A command which can be executed. The command has a name and can have command arguments. | <ul compact><li>`cmd_ignore`</li><li>`cmd_result`</li><li>`namespace`</li><li>`help`</li><li>`args`</li><li>`command`</li></ul> |
///
/// ### Properties
///
/// - test
///
/// ### Properties from components
///
/// - args
/// - cmd_ignore
/// - cmd_result
/// - command
/// - help
/// - namespace
///
/// ## JSON Schema
///
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/command/NumCommands.schema.json]()
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder
)]
pub struct NumCommands {
    #[builder(default, setter(into))]
    pub id: uuid::Uuid,
    
    /// ### Property `args`
    ///
    /// The command arguments
    ///
    /// Data Type: `Array`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    #[builder(setter(into))]
    pub args: Vec<serde_json::Value>,
    
    /// ### Property `cmd_ignore`
    ///
    /// blah
    ///
    /// Data Type: `Any`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    pub cmd_ignore: serde_json::Value,
    
    /// ### Property `cmd_result`
    ///
    /// The result of the command
    ///
    /// Data Type: `Any`
    ///
    /// Socket Type: `Output`
    ///
    /// Mutability: `Immutable`
    ///
    pub cmd_result: serde_json::Value,
    
    /// ### Property `command`
    ///
    /// The command name
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    #[builder(setter(into))]
    pub command: String,
    
    /// ### Property `help`
    ///
    /// Help text which explains the command.
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    #[builder(setter(into))]
    pub help: String,
    
    /// ### Property `namespace`
    ///
    /// The command namespace
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    #[builder(setter(into))]
    pub namespace: String,
    
    /// ### Property `test`
    ///
    /// Data Type: `Bool`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    pub test: bool,
    #[builder(default, setter(into))]
    pub extensions: reactive_graph_graph::Extensions,
}

impl NumCommands {
    pub fn new(
        args: Vec<serde_json::Value>,
        cmd_ignore: serde_json::Value,
        cmd_result: serde_json::Value,
        command: String,
        help: String,
        namespace: String,
        test: bool,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            args,
            cmd_ignore,
            cmd_result,
            command,
            help,
            namespace,
            test,
            extensions: reactive_graph_graph::Extensions::new(),
        }
    }
    pub fn new_with_id(
        id: uuid::Uuid,
        args: Vec<serde_json::Value>,
        cmd_ignore: serde_json::Value,
        cmd_result: serde_json::Value,
        command: String,
        help: String,
        namespace: String,
        test: bool,
    ) -> Self {
        Self {
            id,
            args,
            cmd_ignore,
            cmd_result,
            command,
            help,
            namespace,
            test,
            extensions: reactive_graph_graph::Extensions::new(),
        }
    }
    pub fn new_with_extensions(
        id: uuid::Uuid,
        args: Vec<serde_json::Value>,
        cmd_ignore: serde_json::Value,
        cmd_result: serde_json::Value,
        command: String,
        help: String,
        namespace: String,
        test: bool,
        extensions: reactive_graph_graph::Extensions,
    ) -> Self {
        Self {
            id,
            args,
            cmd_ignore,
            cmd_result,
            command,
            help,
            namespace,
            test,
            extensions,
        }
    }
    
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
    
    /// ### Property `args`
    ///
    /// The command arguments
    ///
    /// Data Type: `Array`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    pub fn args(&self) -> Vec<serde_json::Value> {
        self.args.clone()
    }
    
    /// ### Property `cmd_ignore`
    ///
    /// blah
    ///
    /// Data Type: `Any`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    pub fn cmd_ignore(&self) -> serde_json::Value {
        self.cmd_ignore.clone()
    }
    
    /// ### Property `cmd_ignore`
    ///
    /// blah
    ///
    /// Data Type: `Any`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    pub fn set_cmd_ignore(&mut self, cmd_ignore: serde_json::Value) {
        self.cmd_ignore = cmd_ignore;
    }
    
    /// ### Property `cmd_result`
    ///
    /// The result of the command
    ///
    /// Data Type: `Any`
    ///
    /// Socket Type: `Output`
    ///
    /// Mutability: `Immutable`
    ///
    pub fn cmd_result(&self) -> serde_json::Value {
        self.cmd_result.clone()
    }
    
    /// ### Property `command`
    ///
    /// The command name
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    pub fn command(&self) -> String {
        self.command.clone()
    }
    
    /// ### Property `help`
    ///
    /// Help text which explains the command.
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    pub fn help(&self) -> String {
        self.help.clone()
    }
    
    /// ### Property `namespace`
    ///
    /// The command namespace
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    pub fn namespace(&self) -> String {
        self.namespace.clone()
    }
    
    /// ### Property `test`
    ///
    /// Data Type: `Bool`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    pub fn test(&self) -> bool {
        self.test
    }
    
    /// ### Property `test`
    ///
    /// Data Type: `Bool`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    pub fn set_test(&mut self, test: bool) {
        self.test = test;
    }
    
    pub fn properties(&self) -> reactive_graph_graph::PropertyInstances {
        reactive_graph_graph::PropertyInstances::new()
            .property(NumCommandsProperties::ARGS, self.args.clone())
            .property(NumCommandsProperties::CMD_IGNORE, self.cmd_ignore.clone())
            .property(NumCommandsProperties::CMD_RESULT, self.cmd_result.clone())
            .property(NumCommandsProperties::COMMAND, self.command.clone())
            .property(NumCommandsProperties::HELP, self.help.clone())
            .property(NumCommandsProperties::NAMESPACE, self.namespace.clone())
            .property(NumCommandsProperties::TEST, self.test.clone())
    }
    
    pub fn extensions(&self) -> reactive_graph_graph::Extensions {
        self.extensions.clone()
    }
}

impl From<NumCommands> for reactive_graph_graph::EntityInstance {
    fn from(num_commands: NumCommands) -> Self {
        reactive_graph_graph::EntityInstance::builder()
            .ty(std::ops::Deref::deref(&NUM_COMMANDS))
            .id(num_commands.id())
            .components(NUM_COMMANDS_COMPONENTS.clone())
            .properties(num_commands.properties())
            .build()
    }
}

impl TryFrom<reactive_graph_graph::EntityInstance> for NumCommands {
    type Error = ();
    fn try_from(
        entity_instance: reactive_graph_graph::EntityInstance,
    ) -> Result<Self, Self::Error> {
        Err(())
    }
}
impl crate::reactive_graph::command::command::Command for NumCommands {
    
    /// ### Property `args`
    ///
    /// The command arguments
    ///
    /// Data Type: `Array`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    fn args(&self) -> Vec<serde_json::Value> {
        self.args.clone()
    }
    
    /// ### Property `cmd_ignore`
    ///
    /// blah
    ///
    /// Data Type: `Any`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn cmd_ignore(&self) -> serde_json::Value {
        self.cmd_ignore.clone()
    }
    
    /// ### Property `cmd_ignore`
    ///
    /// blah
    ///
    /// Data Type: `Any`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_cmd_ignore(&mut self, cmd_ignore: serde_json::Value) {
        self.cmd_ignore = cmd_ignore;
    }
    
    /// ### Property `cmd_result`
    ///
    /// The result of the command
    ///
    /// Data Type: `Any`
    ///
    /// Socket Type: `Output`
    ///
    /// Mutability: `Immutable`
    ///
    fn cmd_result(&self) -> serde_json::Value {
        self.cmd_result.clone()
    }
    
    /// ### Property `command`
    ///
    /// The command name
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    fn command(&self) -> String {
        self.command.clone()
    }
    
    /// ### Property `help`
    ///
    /// Help text which explains the command.
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    fn help(&self) -> String {
        self.help.clone()
    }
    
    /// ### Property `namespace`
    ///
    /// The command namespace
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Immutable`
    ///
    fn namespace(&self) -> String {
        self.namespace.clone()
    }
}
