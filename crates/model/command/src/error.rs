#[derive(Debug)]
pub struct NotACommand;

#[derive(Debug)]
pub enum CommandBuilderError {
    NotACommand,
    MissingTrigger,
    MissingExecutor,
}

#[derive(Debug)]
pub struct InvalidCommandArgDefinition(pub serde_json::Error);

impl PartialEq for InvalidCommandArgDefinition {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[derive(Debug, PartialEq)]
pub enum CommandArgsError {
    InvalidCommandArgDefinition(InvalidCommandArgDefinition),
    CommandArgDefinitionMissing,
}

#[derive(Debug, PartialEq)]
pub enum CommandExecutionFailed {
    NotACommand,
    CommandArgsError(CommandArgsError),
    InvalidArgument(String),
    MissingArgumentProperty(String),
    MissingMandatoryArgument(String),
}

#[derive(Debug)]
pub enum NoSuchCommand {
    /// A command with the name wasn't found.
    CommandNotFound(String),
    /// The entity was found but it is not a command.
    NotACommand(NotACommand),
}
