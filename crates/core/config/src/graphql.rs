use serde::Deserialize;
use serde::Serialize;

pub const GRAPHQL_DEFAULT_HOSTNAME: &str = "localhost";
pub const GRAPHQL_DEFAULT_PORT: u16 = 31415;

/// Configuration for the logging middleware of the GraphQL server.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GraphQLLoggingConfig {
    /// If true, a request logging middleware is enabled.
    pub enabled: bool,

    /// The log format of the request logging middleware.
    /// See: <https://docs.rs/actix-web/0.6.0/actix_web/middleware/struct.Logger.html#format>
    pub format: Option<String>,
}

/// Configuration for the GraphQL server.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GraphQLServerConfig {
    /// If false, the GraphQL server will be disabled.
    pub enabled: Option<bool>,

    /// The hostname to bind the GraphQL HTTP server.
    pub hostname: Option<String>,

    /// The port to bind the GraphQL HTTP server.
    pub port: Option<u16>,

    /// If true, HTTPS is enabled.
    pub secure: Option<bool>,

    // TODO: configure HTTPS private/public key location
    /// Timeout for graceful workers shutdown in seconds.
    /// After receiving a stop signal, workers have this much time to finish serving requests.
    /// Workers still alive after the timeout are force dropped.
    /// By default shutdown timeout sets to 30 seconds.
    pub shutdown_timeout: Option<u64>,

    /// The number of workers to start.
    /// The default worker count is the number of physical CPU cores available.
    pub workers: Option<usize>,

    /// The default context path which redirects the root context to a web resource provider.
    pub default_context_path: Option<String>,

    /// The logging middleware configuration.
    pub logging: Option<GraphQLLoggingConfig>,
}

impl GraphQLServerConfig {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or(true)
    }

    pub fn hostname(&self) -> String {
        self.hostname.clone().unwrap_or(String::from(GRAPHQL_DEFAULT_HOSTNAME))
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(GRAPHQL_DEFAULT_PORT)
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.hostname(), self.port())
    }

    pub fn url(&self) -> String {
        format!("{}://{}", self.protocol(), self.addr())
    }

    pub fn protocol(&self) -> &str {
        if self.is_secure() {
            "https"
        } else {
            "http"
        }
    }

    pub fn is_secure(&self) -> bool {
        self.secure.unwrap_or(false)
    }

    pub fn shutdown_timeout(&self) -> u64 {
        self.shutdown_timeout.unwrap_or(30)
    }

    pub fn workers(&self) -> usize {
        self.workers.unwrap_or(num_cpus::get_physical())
    }

    pub fn default_context_path(&self) -> Option<String> {
        self.default_context_path.clone()
    }
}

impl Default for GraphQLServerConfig {
    fn default() -> Self {
        GraphQLServerConfig {
            enabled: None,
            hostname: Some(String::from(GRAPHQL_DEFAULT_HOSTNAME)),
            port: Some(GRAPHQL_DEFAULT_PORT),
            secure: None,
            shutdown_timeout: None,
            workers: None,
            default_context_path: None,
            logging: None,
        }
    }
}
