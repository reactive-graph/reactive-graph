#[cfg(feature = "client")]
use crate::client::args::ClientArguments;
#[cfg(feature = "server")]
use crate::server::args::ServerArguments;
use crate::shared::args::SharedArguments;
#[cfg(feature = "tooling")]
use crate::tooling::args::ToolingArguments;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "reactive-graph", author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct AllInOneArguments {
    #[clap(flatten)]
    pub shared: SharedArguments,

    #[cfg(feature = "server")]
    #[clap(flatten)]
    pub server: ServerArguments,

    #[cfg(feature = "client")]
    #[clap(flatten)]
    pub client: ClientArguments,

    #[cfg(feature = "tooling")]
    #[clap(flatten)]
    pub tooling: ToolingArguments,
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
        let mut cmd = AllInOneArguments::command();
        print_shell_completions(Shell::Zsh, &mut cmd);
    }

    #[test]
    fn test_print_markdown_help() {
        clap_markdown::print_help_markdown::<AllInOneArguments>();
    }
}
