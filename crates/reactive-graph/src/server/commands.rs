#[cfg(target_os = "linux")]
use crate::server::daemon::args::DaemonArguments;
use crate::server::schema::args::SchemaArguments;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ServerCommands {
    /// Runs the server as daemon.
    #[cfg(target_os = "linux")]
    Daemon(DaemonArguments),
    /// Prints the GraphQL schema and exits.
    Schema(SchemaArguments),
}
