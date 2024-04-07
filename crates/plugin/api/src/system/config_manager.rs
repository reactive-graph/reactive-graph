use reactive_graph_config_model::GraphQLServerConfig;
use reactive_graph_config_model::InstanceConfig;
use reactive_graph_config_model::PluginsConfig;
use reactive_graph_config_model::RemotesConfig;
use springtime_di::injectable;

#[injectable]
pub trait ConfigManager: Send + Sync {
    /// Returns the instance configuration.
    fn get_instance_config(&self) -> InstanceConfig;

    /// Returns the GraphQL server configuration.
    fn get_graphql_server_config(&self) -> GraphQLServerConfig;

    /// Returns the plugins configuration.
    fn get_plugins_config(&self) -> PluginsConfig;

    /// Returns the remotes configuration.
    fn get_remotes_config(&self) -> RemotesConfig;
}
