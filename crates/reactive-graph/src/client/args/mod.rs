pub mod connection;

use clap::Parser;

use crate::client::args::connection::ClientConnectionArguments;
use crate::client::commands::ClientCommands;
use crate::shared::args::SharedArguments;

#[derive(Parser, Debug, Clone)]
pub struct ClientArguments {
    #[clap(flatten)]
    pub(crate) connection: ClientConnectionArguments,

    #[command(subcommand)]
    pub(crate) commands: Option<ClientCommands>,
}

#[derive(Parser, Debug)]
#[command(name = "reactive-graph-client", author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ClientAndSharedArguments {
    #[clap(flatten)]
    pub shared: SharedArguments,

    #[clap(flatten)]
    pub client: ClientArguments,
}

// Tests cannot be build if there are errors in the clap configuration (for
// example, multiple arguments with the same name).
#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::completions::print_shell_completions;
    use clap::CommandFactory;
    use clap_complete::Shell;

    #[test]
    fn test_print_completions() {
        let mut cmd = ClientAndSharedArguments::command();
        print_shell_completions(Shell::Zsh, &mut cmd);
    }

    #[test]
    fn test_print_markdown_help() {
        clap_markdown::print_help_markdown::<ClientAndSharedArguments>();
    }
}
