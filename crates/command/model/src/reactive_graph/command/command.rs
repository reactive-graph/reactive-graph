//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::command::Command`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const COMMAND_NAMESPACE: &str = "reactive_graph::command::Command";

/// The [type identifier]() of Component `Command`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::command::Command`
pub static COMMAND: std::sync::LazyLock<reactive_graph_graph::ComponentTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(COMMAND_NAMESPACE).unwrap());

/// The properties of Component `Command`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum CommandProperties {
    
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
}
impl CommandProperties {
    pub fn len() -> usize {
        6usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types.push(CommandProperties::ARGS);
        property_types.push(CommandProperties::CMD_IGNORE);
        property_types.push(CommandProperties::CMD_RESULT);
        property_types.push(CommandProperties::COMMAND);
        property_types.push(CommandProperties::HELP);
        property_types.push(CommandProperties::NAMESPACE);
        property_types
    }
}

impl AsRef<str> for CommandProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {
            CommandProperties::ARGS => "args",
            CommandProperties::CMD_IGNORE => "cmd_ignore",
            CommandProperties::CMD_RESULT => "cmd_result",
            CommandProperties::COMMAND => "command",
            CommandProperties::HELP => "help",
            CommandProperties::NAMESPACE => "namespace",
        }
    }
}

impl From<CommandProperties> for &'static str {
    #[inline]
    fn from(properties: CommandProperties) -> &'static str {
        match properties {
            CommandProperties::ARGS => "args",
            CommandProperties::CMD_IGNORE => "cmd_ignore",
            CommandProperties::CMD_RESULT => "cmd_result",
            CommandProperties::COMMAND => "command",
            CommandProperties::HELP => "help",
            CommandProperties::NAMESPACE => "namespace",
        }
    }
}

impl From<CommandProperties> for String {
    #[inline]
    fn from(properties: CommandProperties) -> String {
        match properties {
            CommandProperties::ARGS => "args".to_owned(),
            CommandProperties::CMD_IGNORE => "cmd_ignore".to_owned(),
            CommandProperties::CMD_RESULT => "cmd_result".to_owned(),
            CommandProperties::COMMAND => "command".to_owned(),
            CommandProperties::HELP => "help".to_owned(),
            CommandProperties::NAMESPACE => "namespace".to_owned(),
        }
    }
}

impl From<CommandProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(properties: CommandProperties) -> reactive_graph_graph::PropertyType {
        match properties {
            CommandProperties::ARGS => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "args",
                    "The command arguments",
                    reactive_graph_graph::DataType::Array,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Immutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommandProperties::CMD_IGNORE => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "cmd_ignore",
                    "blah",
                    reactive_graph_graph::DataType::Any,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommandProperties::CMD_RESULT => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "cmd_result",
                    "The result of the command",
                    reactive_graph_graph::DataType::Any,
                    reactive_graph_graph::SocketType::Output,
                    reactive_graph_graph::Mutability::Immutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommandProperties::COMMAND => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "command",
                    "The command name",
                    reactive_graph_graph::DataType::String,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Immutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommandProperties::HELP => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "help",
                    "Help text which explains the command.",
                    reactive_graph_graph::DataType::String,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Immutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommandProperties::NAMESPACE => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "namespace",
                    "The command namespace",
                    reactive_graph_graph::DataType::String,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Immutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
        }
    }
}

pub struct CommandPropertiesIterator(Option<CommandProperties>);

impl CommandProperties {
    pub fn into_iter() -> CommandPropertiesIterator {
        CommandPropertiesIterator(None)
    }
}

impl Iterator for CommandPropertiesIterator {
    type Item = CommandProperties;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(CommandProperties::ARGS),
            Some(CommandProperties::ARGS) => Some(CommandProperties::CMD_IGNORE),
            Some(CommandProperties::CMD_IGNORE) => Some(CommandProperties::CMD_RESULT),
            Some(CommandProperties::CMD_RESULT) => Some(CommandProperties::COMMAND),
            Some(CommandProperties::COMMAND) => Some(CommandProperties::HELP),
            Some(CommandProperties::HELP) => Some(CommandProperties::NAMESPACE),
            Some(CommandProperties::NAMESPACE) => None,
        };
        self.0.clone()
    }
}

impl IntoIterator for CommandProperties {
    type Item = CommandProperties;
    type IntoIter = CommandPropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        CommandPropertiesIterator(None)
    }
}

impl core::fmt::Display for CommandProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {
            CommandProperties::ARGS => core::fmt::Display::fmt("args", f),
            CommandProperties::CMD_IGNORE => core::fmt::Display::fmt("cmd_ignore", f),
            CommandProperties::CMD_RESULT => core::fmt::Display::fmt("cmd_result", f),
            CommandProperties::COMMAND => core::fmt::Display::fmt("command", f),
            CommandProperties::HELP => core::fmt::Display::fmt("help", f),
            CommandProperties::NAMESPACE => core::fmt::Display::fmt("namespace", f),
        }
    }
}

///
pub static COMMAND_EXTENSIONS: std::sync::LazyLock<reactive_graph_graph::Extensions> = std::sync::LazyLock::new(||
{ reactive_graph_graph::Extensions::new() });

pub static COMMAND_TYPE: std::sync::LazyLock<reactive_graph_graph::Component> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::Component::builder()
        .ty(core::ops::Deref::deref(&COMMAND))
        .description(
            "A command which can be executed. The command has a name and can have command arguments.",
        )
        .properties(CommandProperties::property_types())
        .build()
});

/// # Component `Command`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::command::Command`
///
/// ## Description
///
/// A command which can be executed. The command has a name and can have command arguments.
///
/// ### Properties
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
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/command/Command.schema.json]()
///
pub trait Command {
    
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
    fn args(&self) -> Vec<serde_json::Value>;
    
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
    fn cmd_ignore(&self) -> serde_json::Value;
    
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
    fn set_cmd_ignore(&mut self, cmd_ignore: serde_json::Value);
    
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
    fn cmd_result(&self) -> serde_json::Value;
    
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
    fn command(&self) -> String;
    
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
    fn help(&self) -> String;
    
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
    fn namespace(&self) -> String;
}
