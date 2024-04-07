use clap::Arg;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::error::CommandArgsError;
use crate::error::InvalidCommandArgDefinition;
use reactive_graph_graph::DataType;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_graph::SocketType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandArg {
    /// The argument name.
    /// The argument matches the property name.
    pub name: String,

    /// The short name.
    /// -a 1
    pub short: Option<char>,

    /// The long name of the argument.
    /// --argument=123
    pub long: Option<String>,

    /// The help text.
    pub help: Option<String>,

    /// True, if the command argument is required.
    #[serde(default = "bool::default")]
    pub required: bool,
}

impl CommandArg {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            short: None,
            long: None,
            help: None,
            required: false,
        }
    }

    pub fn short(self, short: char) -> Self {
        Self {
            name: self.name,
            short: Some(short),
            long: self.long,
            help: self.help,
            required: self.required,
        }
    }

    pub fn long<S: Into<String>>(self, long: S) -> Self {
        Self {
            name: self.name,
            short: self.short,
            long: Some(long.into()),
            help: None,
            required: self.required,
        }
    }

    pub fn help<S: Into<String>>(self, help: S) -> Self {
        Self {
            name: self.name,
            short: self.short,
            long: self.long,
            help: Some(help.into()),
            required: self.required,
        }
    }

    pub fn required(self, required: bool) -> Self {
        Self {
            name: self.name,
            short: self.short,
            long: self.long,
            help: self.help,
            required,
        }
    }

    pub fn as_arg(&self) -> Arg {
        let mut arg = Arg::new(self.name.clone());
        if let Some(long) = &self.long {
            arg = arg.long(long.clone());
        }
        if let Some(help) = self.help.clone() {
            arg = arg.help(help);
        }
        arg
    }
}

impl TryFrom<Value> for CommandArg {
    type Error = InvalidCommandArgDefinition;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let arg: CommandArg = serde_json::from_value(value).map_err(InvalidCommandArgDefinition)?;
        Ok(arg)
    }
}

impl From<String> for CommandArg {
    fn from(name: String) -> Self {
        CommandArg::new(name)
    }
}

impl From<&str> for CommandArg {
    fn from(name: &str) -> Self {
        CommandArg::new(name)
    }
}

impl From<CommandArg> for PropertyType {
    fn from(arg: CommandArg) -> Self {
        PropertyType::builder()
            .name(arg.name)
            .description(arg.help.unwrap_or_default())
            .data_type(DataType::Any)
            .socket_type(SocketType::Input)
            .build()
    }
}

#[derive(Debug, Clone)]
pub struct CommandArgs(Vec<CommandArg>);

impl CommandArgs {
    pub fn new() -> Self {
        CommandArgs(Vec::new())
    }

    pub fn arg<A: Into<CommandArg>>(mut self, arg: A) -> Self {
        self.0.push(arg.into());
        self
    }

    pub fn push(&mut self, arg: CommandArg) {
        self.0.push(arg);
    }

    pub fn contains<S: Into<String>>(&self, name: S) -> bool {
        let name = name.into();
        self.0.iter().any(|arg| arg.name == name)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn to_value(&self) -> Value {
        Value::Array(self.0.iter().filter_map(|arg| serde_json::to_value(arg).ok()).collect())
    }

    pub fn as_args(&self) -> Vec<Arg> {
        self.0.iter().map(|arg| arg.as_arg()).collect()
    }

    pub fn to_vec(&self) -> Vec<CommandArg> {
        self.0.to_vec()
    }

    pub fn to_property_types(&self) -> PropertyTypes {
        self.to_vec().iter().map(|arg| arg.clone().into()).collect::<Vec<_>>().into()
    }
}

impl Default for CommandArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Value> for CommandArgs {
    type Error = CommandArgsError;

    fn try_from(args: Value) -> Result<Self, Self::Error> {
        args.as_array()
            .map(|args| {
                // let args: Result<Vec<CommandArg>, InvalidCommandArgDefinition> = args.iter().map(|arg| CommandArg::try_from(arg.clone())).collect();
                match args.iter().map(|arg| CommandArg::try_from(arg.clone())).collect() {
                    Ok(args) => Ok(CommandArgs(args)),
                    Err(e) => Err(CommandArgsError::InvalidCommandArgDefinition(e)),
                }
            })
            .unwrap_or(Err(CommandArgsError::CommandArgDefinitionMissing))
    }
}
