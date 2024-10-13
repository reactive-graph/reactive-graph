use crate::server::args::config_locations::ConfigLocationsArguments;
use crate::server::args::graphql_server::GraphQLServerArguments;
use crate::server::args::instance_config::InstanceConfigArguments;
use crate::server::args::plugins::PluginsArguments;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct RuntimeArguments {
    #[clap(flatten)]
    pub config_locations: ConfigLocationsArguments,

    #[clap(flatten)]
    pub instance_config: InstanceConfigArguments,

    #[clap(flatten)]
    pub graphql_server: GraphQLServerArguments,

    #[clap(flatten)]
    pub plugins: PluginsArguments,

    /// If true, the runtime does not wait before exiting.
    #[arg(long, env = "REACTIVE_GRAPH_STOP_IMMEDIATELY")]
    pub stop_immediately: Option<bool>,
}
