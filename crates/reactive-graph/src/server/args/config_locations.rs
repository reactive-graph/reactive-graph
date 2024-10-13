use clap::Parser;

#[derive(Parser, Debug)]
pub struct ConfigLocationsArguments {
    /// The logging config location.
    #[arg(long, env = "REACTIVE_GRAPH_LOGGING_CONFIG")]
    pub logging_config: Option<String>,

    /// The instance config location.
    #[arg(long, env = "REACTIVE_GRAPH_INSTANCE_CONFIG")]
    pub instance_config: Option<String>,

    /// The GraphQL config location.
    #[arg(long, env = "REACTIVE_GRAPH_GRAPHQL_CONFIG")]
    pub graphql_config: Option<String>,

    /// The plugins config location.
    #[arg(long, env = "REACTIVE_GRAPH_PLUGINS_CONFIG")]
    pub plugins_config: Option<String>,
}
