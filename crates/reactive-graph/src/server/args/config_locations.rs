use clap::Parser;

#[derive(Parser, Debug)]
pub struct ConfigLocationsArguments {
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
}
