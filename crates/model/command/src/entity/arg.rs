use clap::Arg;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::error::InvalidCommandArgDefinition;

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
        let arg: CommandArg = serde_json::from_value(value).map_err(|_| InvalidCommandArgDefinition)?;
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

pub struct CommandArgs(Vec<CommandArg>);

impl CommandArgs {
    pub fn new() -> Self {
        CommandArgs(Vec::new())
    }

    pub fn push(&mut self, arg: CommandArg) {
        self.0.push(arg);
    }

    pub fn contains<S: Into<String>>(&self, name: S) -> bool {
        let name = name.into();
        self.0.iter().find(|arg| arg.name == name).is_some()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_value(&self) -> Value {
        Value::Array(self.0.iter().filter_map(|arg| serde_json::to_value(arg).ok()).collect())
    }

    pub fn as_args(&self) -> Vec<Arg> {
        self.0.iter().map(|arg| arg.as_arg()).into_iter().collect()
    }

    pub fn to_vec(&self) -> Vec<CommandArg> {
        self.0.to_vec()
    }
}

impl TryFrom<Value> for CommandArgs {
    type Error = InvalidCommandArgDefinition;

    fn try_from(args: Value) -> Result<Self, Self::Error> {
        match args.as_array() {
            Some(args) => Ok(CommandArgs(args.iter().filter_map(|arg| CommandArg::try_from(arg.clone()).ok()).collect())),
            None => Err(InvalidCommandArgDefinition),
        }
    }
}
