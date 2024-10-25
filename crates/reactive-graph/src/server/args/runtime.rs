use crate::server::args::config_locations::ConfigLocationsArguments;
use crate::shared::config::graphql::GraphQLServerConfigArgs;
use crate::shared::config::instance::InstanceConfigArgs;
use crate::shared::config::plugins::PluginsConfigArgs;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct RuntimeArguments {
    #[clap(flatten)]
    pub config_locations: ConfigLocationsArguments,

    #[clap(flatten)]
    pub instance: InstanceConfigArgs,

    #[clap(flatten)]
    pub graphql_server: GraphQLServerConfigArgs,

    #[clap(flatten)]
    pub plugins: PluginsConfigArgs,

    /// If true, the runtime does not wait before exiting.
    #[arg(long, env = "REACTIVE_GRAPH_STOP_IMMEDIATELY")]
    pub stop_immediately: Option<bool>,
}
