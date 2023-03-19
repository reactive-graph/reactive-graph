use std::sync::Arc;

use crate::config::GraphQLServerConfig;
use crate::config::InstanceConfig;
use crate::config::PluginsConfig;
use crate::plugins::ConfigManager;

pub struct ConfigManagerImpl {
    config_manager: Arc<dyn crate::api::ConfigManager>,
}

impl ConfigManagerImpl {
    pub fn new(config_manager: Arc<dyn crate::api::ConfigManager>) -> Self {
        Self { config_manager }
    }
}

impl ConfigManager for ConfigManagerImpl {
    fn get_instance_config(&self) -> InstanceConfig {
        self.config_manager.get_instance_config()
    }

    fn get_graphql_server_config(&self) -> GraphQLServerConfig {
        self.config_manager.get_graphql_server_config()
    }

    fn get_plugins_config(&self) -> PluginsConfig {
        self.config_manager.get_plugins_config()
    }
}
