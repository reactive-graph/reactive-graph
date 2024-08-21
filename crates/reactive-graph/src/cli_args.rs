use clap::Parser;
use clap::Subcommand;

#[cfg(feature = "client")]
use crate::cli::args::ClientArgs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "reactive-graph", author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArguments {
    #[command(subcommand)]
    pub(crate) commands: Option<Commands>,

    /// The logging config location.
    #[arg(long, env = "REACTIVE_GRAPH_LOGGING_CONFIG")]
    pub(crate) logging_config: Option<String>,

    /// The instance config location.
    #[arg(long, env = "REACTIVE_GRAPH_INSTANCE_CONFIG")]
    pub(crate) instance_config: Option<String>,

    /// The GraphQL config location.
    #[arg(long, env = "REACTIVE_GRAPH_GRAPHQL_CONFIG")]
    pub(crate) graphql_config: Option<String>,

    /// The plugins config location.
    #[arg(long, env = "REACTIVE_GRAPH_PLUGINS_CONFIG")]
    pub(crate) plugins_config: Option<String>,

    // Instance
    /// The name of the instance.
    #[arg(short = 'n', long, env = "REACTIVE_GRAPH_INSTANCE_NAME")]
    pub(crate) instance_name: Option<String>,

    /// The description of the instance.
    #[arg(short = 'd', long, env = "REACTIVE_GRAPH_INSTANCE_DESCRIPTION")]
    pub(crate) instance_description: Option<String>,

    // GraphQL Server
    /// The hostname to bind the GraphQL HTTP server.
    #[arg(long, env = "REACTIVE_GRAPH_HOSTNAME")]
    pub(crate) hostname: Option<String>,

    /// The port to bind the GraphQL HTTP server.
    #[arg(long, env = "REACTIVE_GRAPH_PORT")]
    pub(crate) port: Option<u16>,

    /// If true, HTTPS is enabled.
    #[arg(long, env = "REACTIVE_GRAPH_SECURE")]
    pub(crate) secure: Option<bool>,

    /// The location of the certificate.
    #[arg(long, env = "REACTIVE_GRAPH_SSL_CERTIFICATE_PATH")]
    pub ssl_certificate_path: Option<String>,

    /// The location of the private key.
    #[arg(long, env = "REACTIVE_GRAPH_SSL_PRIVATE_KEY_PATH")]
    pub ssl_private_key_path: Option<String>,

    /// Timeout for graceful workers shutdown in seconds.
    /// After receiving a stop signal, workers have this much time to finish serving requests.
    /// Workers still alive after the timeout are force dropped.
    /// By default shutdown timeout sets to 30 seconds.
    #[arg(long, env = "REACTIVE_GRAPH_INSTANCE_SHUTDOWN_TIMEOUT")]
    pub(crate) shutdown_timeout: Option<u64>,

    /// The number of workers to start.
    /// The default worker count is the number of physical CPU cores available.
    #[arg(short = 'w', long, env = "REACTIVE_GRAPH_WORKERS")]
    pub(crate) workers: Option<usize>,

    /// The default context path which redirects the root context to a web resource provider.
    #[arg(short = 'c', long, env = "REACTIVE_GRAPH_DEFAULT_CONTEXT_PATH")]
    pub(crate) default_context_path: Option<String>,

    // Plugins
    /// If true, all plugins will be disabled.
    #[arg(short = 'x', long, env = "REACTIVE_GRAPH_DISABLE_ALL_PLUGINS")]
    pub(crate) disable_all_plugins: Option<bool>,

    /// The list of plugins to disable.
    #[arg(short = 'p', long)]
    pub(crate) disabled_plugins: Option<Vec<String>>,

    /// The list of plugins to enable.
    #[arg(short = 'P', long)]
    pub(crate) enabled_plugins: Option<Vec<String>>,

    /// If true, hot deployment will be disabled.
    #[arg(long, env = "REACTIVE_GRAPH_DISABLE_HOT_DEPLOY")]
    pub(crate) disable_hot_deploy: Option<bool>,

    /// The folder which is watched for hot deployment.
    #[arg(long, env = "REACTIVE_GRAPH_HOT_DEPLOY_LOCATION")]
    pub(crate) hot_deploy_location: Option<String>,

    /// The folder which plugins are installed permanently.
    #[arg(long, env = "REACTIVE_GRAPH_INSTALL_LOCATION")]
    pub(crate) install_location: Option<String>,

    /// If true, the runtime does not wait before exiting.
    #[arg(long, env = "REACTIVE_GRAPH_STOP_IMMEDIATELY")]
    pub(crate) stop_immediately: Option<bool>,

    /// If true, logging is disabled completely.
    #[arg(short = 'q', long, env = "REACTIVE_GRAPH_QUIET")]
    pub(crate) quiet: Option<bool>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Connects to a client
    #[cfg(feature = "client")]
    #[non_exhaustive]
    Client(ClientArgs),
}
