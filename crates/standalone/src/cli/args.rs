use clap::ArgAction::SetTrue;
use clap::Args;

use crate::cli::commands::ClientCommands;
use inexor_rgf_remotes_model::InstanceAddress;
use inexor_rgf_remotes_model::DEFAULT_ENDPOINT_DYNAMIC_GRAPH;
use inexor_rgf_remotes_model::DEFAULT_ENDPOINT_GRAPHQL;
use inexor_rgf_remotes_model::DEFAULT_ENDPOINT_PLUGIN;
use inexor_rgf_remotes_model::DEFAULT_ENDPOINT_RUNTIME;
use inexor_rgf_remotes_model::DEFAULT_HOSTNAME;
use inexor_rgf_remotes_model::DEFAULT_PORT;

#[derive(Args, Debug, Clone)]
pub struct ClientArgs {
    /// The hostname to connect to.
    #[arg(long)]
    hostname: Option<String>,

    /// The port to connect to.
    #[arg(long)]
    port: Option<u16>,

    /// If true, connects via HTTPS.
    #[arg(long, action = SetTrue)]
    secure: Option<bool>,

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

    #[command(subcommand)]
    pub(crate) commands: Option<ClientCommands>,
}

impl From<&ClientArgs> for InstanceAddress {
    fn from(client_args: &ClientArgs) -> Self {
        InstanceAddress::builder()
            .hostname(client_args.hostname.clone().unwrap_or(DEFAULT_HOSTNAME.to_string()))
            .port(client_args.port.unwrap_or(DEFAULT_PORT))
            .secure(client_args.secure.unwrap_or_default())
            .endpoint_graphql(client_args.endpoint_graphql.clone().unwrap_or(DEFAULT_ENDPOINT_GRAPHQL.to_string()))
            .endpoint_dynamic_graph(client_args.endpoint_dynamic_graph.clone().unwrap_or(DEFAULT_ENDPOINT_DYNAMIC_GRAPH.to_string()))
            .endpoint_runtime(client_args.endpoint_runtime.clone().unwrap_or(DEFAULT_ENDPOINT_RUNTIME.to_string()))
            .endpoint_plugins(client_args.endpoint_plugins.clone().unwrap_or(DEFAULT_ENDPOINT_PLUGIN.to_string()))
            .bearer(client_args.bearer.clone())
            .build()
    }
}
