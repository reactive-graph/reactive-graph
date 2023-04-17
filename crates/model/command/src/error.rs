#[derive(Debug)]
pub struct NotACommand;

#[derive(Debug)]
pub enum CommandBuilderError {
    NotACommand,
    MissingTrigger,
    MissingExecutor,
}

#[derive(Debug)]
pub enum CommandExecutionFailed {
    NotACommand,
}

#[derive(Debug)]
pub struct InvalidCommandArgDefinition;

#[derive(Debug)]
pub enum CommandArgsError {
    InvalidCommandArgDefinition(InvalidCommandArgDefinition),
    CommandArgDefinitionMissing,
}

#[derive(Debug)]
pub enum NoSuchCommand {
    /// A command with the name wasn't found.
    CommandNotFound(String),
    /// The entity was found but it is not a command.
    NotACommand(NotACommand),
}
