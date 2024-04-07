use std::sync::Arc;

use reactive_graph_config_model::GraphQLServerConfig;
use reactive_graph_config_model::InstanceConfig;
use reactive_graph_config_model::PluginsConfig;
use reactive_graph_config_model::RemotesConfig;

pub struct ConfigManagerDelegate {
    config_manager: Arc<dyn reactive_graph_config_api::ConfigManager + Send + Sync>,
}

impl ConfigManagerDelegate {
    pub fn new(config_manager: Arc<dyn reactive_graph_config_api::ConfigManager + Send + Sync>) -> Self {
        Self { config_manager }
    }
}

impl reactive_graph_plugin_api::ConfigManager for ConfigManagerDelegate {
    fn get_instance_config(&self) -> InstanceConfig {
        self.config_manager.get_instance_config()
    }

    fn get_graphql_server_config(&self) -> GraphQLServerConfig {
        self.config_manager.get_graphql_server_config()
    }

    fn get_plugins_config(&self) -> PluginsConfig {
        self.config_manager.get_plugins_config()
    }

    fn get_remotes_config(&self) -> RemotesConfig {
        self.config_manager.get_remotes_config()
    }
}
