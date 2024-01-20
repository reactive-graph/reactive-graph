use async_graphql::*;

use inexor_rgf_command_model::entity::Command;
use inexor_rgf_command_model::entity::CommandArg;

pub struct GraphQLCommand {
    /// The instance information.
    pub command: Command,
}

#[Object(name = "Command")]
impl GraphQLCommand {
    async fn namespace(&self) -> Option<String> {
        self.command.namespace()
    }

    async fn name(&self) -> Option<String> {
        self.command.name()
    }

    async fn help(&self) -> Option<String> {
        self.command.help()
    }

    async fn arguments(&self, name: Option<String>) -> Vec<GraphQLCommandArgument> {
        match self.command.args() {
            Ok(args) => args
                .to_vec()
                .into_iter()
                .filter_map(|arg| match name.clone() {
                    Some(name) => {
                        if name == arg.name {
                            Some(GraphQLCommandArgument { arg })
                        } else {
                            None
                        }
                    }
                    None => Some(GraphQLCommandArgument { arg }),
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    }
}

pub struct GraphQLCommandArgument {
    /// The instance information.
    pub arg: CommandArg,
}

#[Object(name = "CommandArgument")]
impl GraphQLCommandArgument {
    async fn name(&self) -> String {
        self.arg.name.clone()
    }

    async fn short(&self) -> Option<String> {
        self.arg.short.map(|c| c.to_string())
    }

    async fn long(&self) -> Option<String> {
        self.arg.long.clone()
    }

    async fn help(&self) -> Option<String> {
        self.arg.help.clone()
    }

    async fn required(&self) -> bool {
        self.arg.required
    }
}
