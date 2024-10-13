use clap::Parser;

#[derive(Parser, Debug)]
pub struct DaemonArguments {
    /// Sets the name of the daemon.
    #[arg(long, env = "REACTIVE_GRAPH_DAEMON_NAME")]
    pub(crate) daemon_name: Option<String>,

    /// The location of the daemon PID file.
    /// By default, no PID file will be created.
    #[arg(long, env = "REACTIVE_GRAPH_DAEMON_PID")]
    pub(crate) daemon_pid: Option<String>,

    /// The working directory of the daemon.
    #[arg(long, env = "REACTIVE_GRAPH_DAEMON_WORKING_DIRECTORY")]
    pub(crate) daemon_working_directory: Option<String>,

    /// Stdout will be written into this file.
    #[arg(long, env = "REACTIVE_GRAPH_DAEMON_STDOUT")]
    pub(crate) daemon_stdout: Option<String>,

    /// Stderr will be written into this file.
    #[arg(long, env = "REACTIVE_GRAPH_DAEMON_STDERR")]
    pub(crate) daemon_stderr: Option<String>,

    /// If set will drop privileges to the specified user.
    /// Note: Both must be given: user and group.
    #[arg(long, env = "REACTIVE_GRAPH_DAEMON_USER")]
    pub(crate) daemon_user: Option<String>,

    /// If set will drop privileges to the specified group.
    /// Note: Both must be given: user and group.
    #[arg(long, env = "REACTIVE_GRAPH_DAEMON_GROUP")]
    pub(crate) daemon_group: Option<String>,
}
