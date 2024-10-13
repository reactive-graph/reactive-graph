use crate::shared::manpages::args::ManPagesActionArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub(crate) enum ManPagesCommands {
    /// Prints or installs man pages.
    ManPages(ManPagesActionArgs),
}

#[derive(Subcommand, Debug)]
pub enum ManPagesActionCommands {
    /// Prints the man pages to stdout.
    Print,

    /// Installs the man pages.
    #[cfg(target_os = "linux")]
    Install,
}
