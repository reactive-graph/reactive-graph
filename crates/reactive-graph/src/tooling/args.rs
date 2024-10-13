use clap::Parser;

use crate::shared::args::SharedArguments;
use crate::tooling::commands::ToolingCommands;

#[derive(Parser, Debug)]
pub struct ToolingArguments {
    // #[clap(flatten)]
    // pub(crate) connection: ClientConnectionArguments,
    #[command(subcommand)]
    pub commands: Option<ToolingCommands>,
}

#[derive(Parser, Debug)]
#[command(name = "reactive-graph-tooling", author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ToolingAndSharedArguments {
    #[clap(flatten)]
    pub shared: SharedArguments,

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
        let mut cmd = ToolingAndSharedArguments::command();
        print_shell_completions(Shell::Zsh, &mut cmd);
    }

    #[test]
    fn test_print_markdown_help() {
        clap_markdown::print_help_markdown::<ToolingAndSharedArguments>();
    }
}
