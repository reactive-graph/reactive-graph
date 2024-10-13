pub mod config_locations;
pub mod graphql_server;
pub mod instance_config;
pub mod logging;
pub mod plugins;
pub mod runtime;

use crate::server::args::runtime::RuntimeArguments;
use crate::server::commands::ServerCommands;
use crate::shared::args::SharedArguments;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct ServerArguments {
    #[command(subcommand)]
    pub commands: Option<ServerCommands>,

    #[clap(flatten)]
    pub runtime: RuntimeArguments,

    /// If true, logging is disabled completely.
    #[arg(short = 'q', long, env = "REACTIVE_GRAPH_QUIET")]
    pub quiet: Option<bool>,
}

#[derive(Parser, Debug)]
#[command(name = "reactive-graph-server", author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ServerAndSharedArguments {
    #[clap(flatten)]
    pub shared: SharedArguments,

    // #[cfg(feature = "server")]
    #[clap(flatten)]
    pub server: ServerArguments,
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
        let mut cmd = ServerAndSharedArguments::command();
        print_shell_completions(Shell::Zsh, &mut cmd);
    }

    #[test]
    fn test_print_markdown_help() {
        clap_markdown::print_help_markdown::<ServerAndSharedArguments>();
    }
}
