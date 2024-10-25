use crate::shared::config::graphql::GraphQLServerConfigArgs;
use crate::shared::config::instance::InstanceConfigArgs;
use crate::shared::config::plugins::PluginsConfigArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum InstanceConfigCommands {
    /// Configures the GraphQL server.
    Graphql(GraphQLServerConfigArgs),
    /// Configures the instance.
    Instance(InstanceConfigArgs),
    /// Configures the instance.
    Plugins(PluginsConfigArgs),
}
