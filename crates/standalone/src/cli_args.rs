use clap::ArgAction::SetTrue;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "inexor-rgf-rt-standalone", author, version, about, long_about = None)]
pub struct CliArguments {
    /// The logging config location.
    #[arg(long)]
    pub(crate) logging_config: Option<String>,

    /// The instance config location.
    #[arg(long)]
    pub(crate) instance_config: Option<String>,

    /// The GraphQL config location.
    #[arg(long)]
    pub(crate) graphql_config: Option<String>,

    /// The plugins config location.
    #[arg(long)]
    pub(crate) plugins_config: Option<String>,

    // Instance
    /// The name of the instance.
    #[arg(short = 'n', long)]
    pub(crate) instance_name: Option<String>,

    /// The description of the instance.
    #[arg(short = 'd', long)]
    pub(crate) instance_description: Option<String>,

    // GraphQL Server
    /// The hostname to bind the GraphQL HTTP server.
    #[arg(long)]
    pub(crate) hostname: Option<String>,

    /// The port to bind the GraphQL HTTP server.
    #[arg(long)]
    pub(crate) port: Option<u16>,

    /// If true, HTTPS is enabled.
    #[arg(long, action = SetTrue)]
    pub(crate) secure: Option<bool>,

    /// Timeout for graceful workers shutdown in seconds.
    /// After receiving a stop signal, workers have this much time to finish serving requests.
    /// Workers still alive after the timeout are force dropped.
    /// By default shutdown timeout sets to 30 seconds.
    #[arg(long)]
    pub(crate) shutdown_timeout: Option<u64>,

    /// The number of workers to start.
    /// The default worker count is the number of physical CPU cores available.
    #[arg(short = 'w', long)]
    pub(crate) workers: Option<usize>,

    /// The default context path which redirects the root context to a web resource provider.
    #[arg(short = 'c', long)]
    pub(crate) default_context_path: Option<String>,

    // Plugins
    /// If true, all plugins will be disabled.
    #[arg(short = 'x', long, action = SetTrue)]
    pub(crate) disable_all_plugins: Option<bool>,

    /// The list of plugins to disable.
    #[arg(short = 'p', long)]
    pub(crate) disabled_plugins: Option<Vec<String>>,

    /// If true, hot deployment will be disabled.
    #[arg(long, action = SetTrue)]
    pub(crate) disable_hot_deploy: Option<bool>,

    /// If true, the runtime does not wait before exiting.
    #[arg(long, action = SetTrue)]
    pub(crate) stop_immediately: Option<bool>,

    /// If true, logging is disabled completely.
    #[arg(short = 'q', long, action = SetTrue)]
    pub(crate) quiet: Option<bool>,
}
