use clap::Parser;
use reactive_graph_remotes_model::DEFAULT_ENDPOINT_DYNAMIC_GRAPH;
use reactive_graph_remotes_model::DEFAULT_ENDPOINT_GRAPHQL;
use reactive_graph_remotes_model::DEFAULT_ENDPOINT_PLUGIN;
use reactive_graph_remotes_model::DEFAULT_ENDPOINT_RUNTIME;
use reactive_graph_remotes_model::DEFAULT_HOSTNAME;
use reactive_graph_remotes_model::DEFAULT_PORT;
use reactive_graph_remotes_model::InstanceAddress;

#[derive(Parser, Debug, Clone)]
pub struct ClientConnectionArguments {
    /// The hostname to connect to.
    #[arg(long)]
    client_hostname: Option<String>,

    /// The port to connect to.
    #[arg(long)]
    client_port: Option<u16>,

    /// If true, connects via HTTPS.
    #[arg(long)]
    client_secure: Option<bool>,

    /// The endpoint to use.
    #[arg(long)]
    endpoint_graphql: Option<String>,

    /// The endpoint to use.
    #[arg(long)]
    endpoint_dynamic_graph: Option<String>,

    /// The endpoint to use.
    #[arg(long)]
    endpoint_runtime: Option<String>,

    /// The endpoint to use.
    #[arg(long)]
    endpoint_plugins: Option<String>,

    /// The authentication token.
    #[arg(long)]
    bearer: Option<String>,
}

impl From<&ClientConnectionArguments> for InstanceAddress {
    fn from(args: &ClientConnectionArguments) -> Self {
        InstanceAddress::builder()
            .hostname(args.client_hostname.clone().unwrap_or(DEFAULT_HOSTNAME.to_string()))
            .port(args.client_port.unwrap_or(DEFAULT_PORT))
            .secure(args.client_secure.unwrap_or_default())
            .endpoint_graphql(args.endpoint_graphql.clone().unwrap_or(DEFAULT_ENDPOINT_GRAPHQL.to_string()))
            .endpoint_dynamic_graph(args.endpoint_dynamic_graph.clone().unwrap_or(DEFAULT_ENDPOINT_DYNAMIC_GRAPH.to_string()))
            .endpoint_runtime(args.endpoint_runtime.clone().unwrap_or(DEFAULT_ENDPOINT_RUNTIME.to_string()))
            .endpoint_plugin(args.endpoint_plugins.clone().unwrap_or(DEFAULT_ENDPOINT_PLUGIN.to_string()))
            .bearer(args.bearer.clone())
            .build()
    }
}
