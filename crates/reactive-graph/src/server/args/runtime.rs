use crate::server::args::config_locations::ConfigLocationsArguments;
use crate::server::args::graphql_server::GraphQLServerArguments;
use crate::server::args::instance_config::InstanceConfigArguments;
use crate::server::args::plugins::PluginsArguments;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct RuntimeArguments {
    #[clap(flatten)]
    pub(crate) config_locations: ConfigLocationsArguments,

    #[clap(flatten)]
    pub(crate) instance_config: InstanceConfigArguments,

    #[clap(flatten)]
    pub(crate) graphql_server: GraphQLServerArguments,

    #[clap(flatten)]
    pub(crate) plugins: PluginsArguments,

    /// If true, the runtime does not wait before exiting.
    #[arg(long, env = "REACTIVE_GRAPH_STOP_IMMEDIATELY")]
    pub stop_immediately: Option<bool>,
}
