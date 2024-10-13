use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum MarkdownHelpCommands {
    /// Prints the markdown help to stdout
    PrintMarkdownHelp,
}
