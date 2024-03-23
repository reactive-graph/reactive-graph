use std::path::PathBuf;

use springtime_di::injectable;

use inexor_rgf_config_model::GraphQLServerConfig;
use inexor_rgf_config_model::InstanceConfig;
use inexor_rgf_config_model::PluginsConfig;
use inexor_rgf_config_model::RemotesConfig;
use inexor_rgf_lifecycle::Lifecycle;

#[injectable]
pub trait ConfigManager: Send + Sync + Lifecycle {
    /// Returns the location of the configuration of the instance.
    fn get_instance_config_location(&self) -> PathBuf;

    /// Sets the location of the configuration of the instance.
    fn set_instance_config_location(&self, instance_config_location: PathBuf);

    /// Returns the location of the configuration of the GraphQL server.
    fn get_graphql_server_config_location(&self) -> PathBuf;

    /// Sets the location of the configuration of the GraphQL server.
    fn set_graphql_server_config_location(&self, graphql_server_config_location: PathBuf);

    /// Returns the location of the plugins configuration.
    fn get_plugins_config_location(&self) -> PathBuf;

    /// Sets the location of the plugins configuration.
    fn set_plugins_config_location(&self, plugins_config_location: PathBuf);

    /// Returns the location of the remotes configuration.
    fn get_remotes_config_location(&self) -> PathBuf;

    /// Sets the location of the remotes configuration.
    fn set_remotes_config_location(&self, remotes_config_location: PathBuf);

    /// Returns the configuration of the instance.
    fn get_instance_config(&self) -> InstanceConfig;

    /// Sets the configuration of the instance.
    fn set_instance_config(&self, instance_config: InstanceConfig);

    /// Reads the configuration of the instance from file.
    fn read_instance_config(&self);

    /// Sets the instance name.
    fn set_instance_name(&self, instance_name: &str);

    /// Sets the instance description.
    fn set_instance_description(&self, instance_description: &str);

    /// Returns the configuration of the GraphQL server.
    fn get_graphql_server_config(&self) -> GraphQLServerConfig;

    /// Sets the configuration of the GraphQL server.
    fn set_graphql_server_config(&self, graphql_server_config: GraphQLServerConfig);

    /// Reads the configuration of the GraphQL server from file.
    fn read_graphql_server_config(&self);

    /// Sets the host name.
    fn set_graphql_hostname(&self, hostname: &str);

    /// Sets the port.
    fn set_graphql_port(&self, port: u16);

    /// Enables / disables HTTPS.
    fn set_graphql_secure(&self, secure: bool);

    /// Sets the SSL certificate path.
    fn set_graphql_ssl_certificate_path(&self, ssl_certificate_path: &str);

    /// Sets the SSL private key path.
    fn set_graphql_ssl_private_key_path(&self, ssl_private_key_path: &str);

    /// Sets the timeout for graceful workers shutdown in seconds.
    fn set_graphql_shutdown_timeout(&self, shutdown_timeout: u64);

    /// Sets the number of workers.
    fn set_graphql_workers(&self, workers: usize);

    /// Returns the default context path which redirects the root context to a web resource provider.
    fn get_graphql_default_context_path(&self) -> Option<String>;

    /// Sets the default context path which redirects the root context to a web resource provider.
    fn set_graphql_default_context_path(&self, default_context_path: String);

    /// Returns the plugins configuration.
    fn get_plugins_config(&self) -> PluginsConfig;

    /// Sets the plugins configuration.
    fn set_plugins_config(&self, plugins_config: PluginsConfig);

    /// Reads the plugins configuration from file.
    fn read_plugins_config(&self);

    /// Enables / disables all plugins.
    fn set_disable_all_plugins(&self, disable_all_plugins: bool);

    /// Sets the plugins to disable.
    fn set_disabled_plugins(&self, disabled_plugins: Vec<String>);

    /// Sets the plugins to enable. If set, set_disabled_plugins will have no effect.
    fn set_enabled_plugins(&self, enabled_plugins: Vec<String>);

    /// Enables / disables hot deploy.
    fn set_disable_hot_deploy(&self, disable_hot_deploy: bool);

    /// Sets the plugins hot deploy location.
    fn set_hot_deploy_location(&self, hot_deploy_location: Option<String>);

    /// Sets the plugins install location.
    fn set_install_location(&self, install_location: Option<String>);

    /// Returns the remotes.
    fn get_remotes_config(&self) -> RemotesConfig;

    /// Sets the remotes configuration.
    fn set_remotes_config(&self, remotes: RemotesConfig);

    /// Reads the remotes configuration from file.
    fn read_remotes_config(&self);

    /// Writes the remotes configuration to file.
    fn write_remotes_config(&self);
}
