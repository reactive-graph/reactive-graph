use crate::config::GraphQLServerConfig;
use crate::config::InstanceConfig;
use crate::config::PluginsConfig;

pub trait ConfigManager: Send + Sync {
    /// Returns the instance configuration.
    fn get_instance_config(&self) -> InstanceConfig;

    /// Returns the GraphQL server configuration.
    fn get_graphql_server_config(&self) -> GraphQLServerConfig;

    /// Returns the plugins configuration.
    fn get_plugins_config(&self) -> PluginsConfig;
}
