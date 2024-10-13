use clap::Parser;

#[derive(Parser, Debug)]
pub struct GraphQLServerArguments {
    // GraphQL Server
    /// The hostname to bind the GraphQL HTTP server.
    #[arg(long, env = "REACTIVE_GRAPH_HOSTNAME")]
    pub hostname: Option<String>,

    /// The port to bind the GraphQL HTTP server.
    #[arg(long, env = "REACTIVE_GRAPH_PORT")]
    pub port: Option<u16>,

    /// If true, HTTPS is enabled.
    #[arg(long, env = "REACTIVE_GRAPH_SECURE")]
    pub secure: Option<bool>,

    /// The location of the certificate.
    #[arg(long, env = "REACTIVE_GRAPH_SSL_CERTIFICATE_PATH")]
    pub ssl_certificate_path: Option<String>,

    /// The location of the private key.
    #[arg(long, env = "REACTIVE_GRAPH_SSL_PRIVATE_KEY_PATH")]
    pub ssl_private_key_path: Option<String>,

    /// Timeout for graceful workers shutdown in seconds.
    /// After receiving a stop signal, workers have this much time to finish serving requests.
    /// Workers still alive after the timeout are force dropped.
    /// By default, shutdown timeout sets to 30 seconds.
    #[arg(long, env = "REACTIVE_GRAPH_INSTANCE_SHUTDOWN_TIMEOUT")]
    pub shutdown_timeout: Option<u64>,

    /// The number of workers to start.
    /// The default worker count is the number of physical CPU cores available.
    #[arg(short = 'w', long, env = "REACTIVE_GRAPH_WORKERS")]
    pub workers: Option<usize>,

    /// The default context path which redirects the root context to a web resource provider.
    #[arg(short = 'c', long, env = "REACTIVE_GRAPH_DEFAULT_CONTEXT_PATH")]
    pub default_context_path: Option<String>,
}
