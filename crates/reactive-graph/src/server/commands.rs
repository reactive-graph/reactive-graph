use crate::server::daemon::args::DaemonArguments;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ServerCommands {
    /// Runs the server as daemon.
    #[cfg(target_os = "linux")]
    Daemon(DaemonArguments),
}
