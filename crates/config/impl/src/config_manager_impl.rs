use std::path::PathBuf;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use log::error;
use log::info;
use springtime_di::component_alias;
use springtime_di::Component;

use inexor_rgf_config_api::ConfigManager;
use inexor_rgf_config_model::GraphQLServerConfig;
use inexor_rgf_config_model::InstanceConfig;
use inexor_rgf_config_model::PluginsConfig;
use inexor_rgf_config_model::RemotesConfig;
use inexor_rgf_lifecycle::Lifecycle;

const DEFAULT_CONFIG_LOCATION: &str = "./config";

const DEFAULT_INSTANCE_CONFIG_FILENAME: &str = "instance.toml";

const DEFAULT_GRAPHQL_CONFIG_FILENAME: &str = "graphql.toml";

const DEFAULT_PLUGINS_CONFIG_FILENAME: &str = "plugins.toml";

const DEFAULT_REMOTES_CONFIG_FILENAME: &str = "remotes.toml";

fn create_instance_config_location() -> RwLock<PathBuf> {
    // InstanceConfigLocation {
    let mut p = PathBuf::from(DEFAULT_CONFIG_LOCATION);
    p.push(DEFAULT_INSTANCE_CONFIG_FILENAME);
    RwLock::new(p)
}

fn create_graphql_server_config_location() -> RwLock<PathBuf> {
    let mut p = PathBuf::from(DEFAULT_CONFIG_LOCATION);
    p.push(DEFAULT_GRAPHQL_CONFIG_FILENAME);
    RwLock::new(p)
}

fn create_plugins_config_location() -> RwLock<PathBuf> {
    let mut p = PathBuf::from(DEFAULT_CONFIG_LOCATION);
    p.push(DEFAULT_PLUGINS_CONFIG_FILENAME);
    RwLock::new(p)
}

fn create_remotes_config_location() -> RwLock<PathBuf> {
    let mut p = PathBuf::from(DEFAULT_CONFIG_LOCATION);
    p.push(DEFAULT_REMOTES_CONFIG_FILENAME);
    RwLock::new(p)
}

fn create_instance_config() -> RwLock<InstanceConfig> {
    RwLock::new(InstanceConfig::default())
}

fn create_graphql_server_config() -> RwLock<GraphQLServerConfig> {
    RwLock::new(GraphQLServerConfig::default())
}

fn create_plugins_config() -> RwLock<PluginsConfig> {
    RwLock::new(PluginsConfig::default())
}

fn create_remotes_config() -> RwLock<RemotesConfig> {
    RwLock::new(RemotesConfig::default())
}

#[derive(Component)]
pub struct ConfigManagerImpl {
    #[component(default = "create_instance_config_location")]
    instance_config_location: RwLock<PathBuf>,
    #[component(default = "create_graphql_server_config_location")]
    graphql_server_config_location: RwLock<PathBuf>,
    #[component(default = "create_plugins_config_location")]
    plugins_config_location: RwLock<PathBuf>,
    #[component(default = "create_remotes_config_location")]
    remotes_config_location: RwLock<PathBuf>,
    #[component(default = "create_instance_config")]
    instance_config: RwLock<InstanceConfig>,
    #[component(default = "create_graphql_server_config")]
    graphql_server_config: RwLock<GraphQLServerConfig>,
    #[component(default = "create_plugins_config")]
    plugins_config: RwLock<PluginsConfig>,
    #[component(default = "create_remotes_config")]
    remotes_config: RwLock<RemotesConfig>,
}

// #[async_trait]
#[component_alias]
impl ConfigManager for ConfigManagerImpl {
    fn get_instance_config_location(&self) -> PathBuf {
        let reader = self.instance_config_location.read().unwrap();
        reader.clone()
    }

    fn set_instance_config_location(&self, instance_config_location: PathBuf) {
        let mut writer = self.instance_config_location.write().unwrap();
        *writer = instance_config_location;
    }

    fn get_graphql_server_config_location(&self) -> PathBuf {
        let reader = self.graphql_server_config_location.read().unwrap();
        reader.clone()
    }

    fn set_graphql_server_config_location(&self, graphql_server_config_location: PathBuf) {
        let mut writer = self.graphql_server_config_location.write().unwrap();
        *writer = graphql_server_config_location;
    }

    fn get_plugins_config_location(&self) -> PathBuf {
        let reader = self.plugins_config_location.read().unwrap();
        reader.clone()
    }

    fn set_plugins_config_location(&self, plugins_config_location: PathBuf) {
        let mut writer = self.plugins_config_location.write().unwrap();
        *writer = plugins_config_location;
    }

    fn get_remotes_config_location(&self) -> PathBuf {
        let reader = self.remotes_config_location.read().unwrap();
        reader.clone()
    }

    fn set_remotes_config_location(&self, remotes_config_location: PathBuf) {
        let mut writer = self.remotes_config_location.write().unwrap();
        *writer = remotes_config_location;
    }

    fn get_instance_config(&self) -> InstanceConfig {
        let reader = self.instance_config.read().unwrap();
        reader.clone()
    }

    fn set_instance_config(&self, instance_config: InstanceConfig) {
        let mut writer = self.instance_config.write().unwrap();
        *writer = instance_config;
    }

    fn read_instance_config(&self) {
        let location = self.get_instance_config_location();
        match std::fs::read_to_string(&location) {
            Ok(toml_string) => match toml::from_str(&toml_string) {
                Ok(instance_config) => {
                    self.set_instance_config(instance_config);
                }
                Err(_) => {
                    error!("Failed to load the instance configuration from {}: Invalid TOML", location.to_str().unwrap_or(""));
                }
            },
            Err(_) => {
                error!("Failed to load the instance configuration from {}: File does not exist", location.to_str().unwrap_or(""));
            }
        }
    }

    fn set_instance_name(&self, instance_name: &str) {
        let mut writer = self.instance_config.write().unwrap();
        writer.name = instance_name.to_string();
    }

    fn set_instance_description(&self, instance_description: &str) {
        let mut writer = self.instance_config.write().unwrap();
        writer.description = instance_description.to_string();
    }

    fn get_graphql_server_config(&self) -> GraphQLServerConfig {
        let reader = self.graphql_server_config.read().unwrap();
        reader.clone()
    }

    fn set_graphql_server_config(&self, graphql_server_config: GraphQLServerConfig) {
        let mut writer = self.graphql_server_config.write().unwrap();
        *writer = graphql_server_config;
    }

    fn read_graphql_server_config(&self) {
        let location = self.get_graphql_server_config_location();
        match std::fs::read_to_string(&location) {
            Ok(toml_string) => match toml::from_str(&toml_string) {
                Ok(graphql_server_config) => {
                    debug!("Loaded graphql configuration from {}", location.to_str().unwrap_or(""));
                    self.set_graphql_server_config(graphql_server_config);
                }
                Err(_) => {
                    error!("Failed to load the graphql configuration from {}: Invalid TOML", location.to_str().unwrap_or(""));
                }
            },
            Err(_) => {
                error!("Failed to load the graphql configuration from {}: File does not exist", location.to_str().unwrap_or(""));
            }
        }
    }

    fn set_graphql_hostname(&self, hostname: &str) {
        let mut writer = self.graphql_server_config.write().unwrap();
        writer.hostname = Some(String::from(hostname));
    }

    fn set_graphql_port(&self, port: u16) {
        let mut writer = self.graphql_server_config.write().unwrap();
        writer.port = Some(port);
    }

    fn set_graphql_secure(&self, secure: bool) {
        let mut writer = self.graphql_server_config.write().unwrap();
        writer.secure = Some(secure);
    }

    fn set_graphql_ssl_certificate_path(&self, ssl_certificate_path: &str) {
        let mut writer = self.graphql_server_config.write().unwrap();
        writer.ssl_certificate_path = Some(String::from(ssl_certificate_path));
    }

    fn set_graphql_ssl_private_key_path(&self, ssl_private_key_path: &str) {
        let mut writer = self.graphql_server_config.write().unwrap();
        writer.ssl_private_key_path = Some(String::from(ssl_private_key_path));
    }

    fn set_graphql_shutdown_timeout(&self, shutdown_timeout: u64) {
        let mut writer = self.graphql_server_config.write().unwrap();
        writer.shutdown_timeout = Some(shutdown_timeout);
    }

    fn set_graphql_workers(&self, workers: usize) {
        let mut writer = self.graphql_server_config.write().unwrap();
        writer.workers = Some(workers);
    }

    fn get_graphql_default_context_path(&self) -> Option<String> {
        self.graphql_server_config.read().unwrap().default_context_path()
    }

    fn set_graphql_default_context_path(&self, default_context_path: String) {
        let mut writer = self.graphql_server_config.write().unwrap();
        writer.default_context_path = Some(default_context_path);
    }

    fn get_plugins_config(&self) -> PluginsConfig {
        let reader = self.plugins_config.read().unwrap();
        reader.clone()
    }

    fn set_plugins_config(&self, plugins_config: PluginsConfig) {
        let mut writer = self.plugins_config.write().unwrap();
        *writer = plugins_config;
    }

    fn read_plugins_config(&self) {
        let location = self.get_plugins_config_location();
        match std::fs::read_to_string(&location) {
            Ok(toml_string) => match toml::from_str(&toml_string) {
                Ok(plugins_config) => {
                    self.set_plugins_config(plugins_config);
                }
                Err(_) => {
                    error!("Failed to load the plugins configuration from {}: Invalid TOML", location.to_str().unwrap_or(""));
                }
            },
            Err(_) => {
                error!("Failed to load the plugins configuration from {}: File does not exist", location.to_str().unwrap_or(""));
            }
        }
    }

    fn set_disable_all_plugins(&self, disabled: bool) {
        let mut writer = self.plugins_config.write().unwrap();
        writer.disabled = Some(disabled);
    }

    fn set_disabled_plugins(&self, disabled_plugins: Vec<String>) {
        let mut writer = self.plugins_config.write().unwrap();
        writer.disabled_plugins = Some(disabled_plugins);
    }

    fn set_enabled_plugins(&self, enabled_plugins: Vec<String>) {
        let mut writer = self.plugins_config.write().unwrap();
        writer.enabled_plugins = Some(enabled_plugins);
    }

    fn set_disable_hot_deploy(&self, disable_hot_deploy: bool) {
        let mut writer = self.plugins_config.write().unwrap();
        writer.hot_deploy = Some(!disable_hot_deploy);
    }

    fn set_hot_deploy_location(&self, hot_deploy_location: Option<String>) {
        let mut writer = self.plugins_config.write().unwrap();
        writer.hot_deploy_location = hot_deploy_location;
    }

    fn set_install_location(&self, install_location: Option<String>) {
        let mut writer = self.plugins_config.write().unwrap();
        writer.install_location = install_location;
    }

    fn get_remotes_config(&self) -> RemotesConfig {
        let reader = self.remotes_config.read().unwrap();
        reader.clone()
    }

    fn set_remotes_config(&self, remotes_config: RemotesConfig) {
        let mut writer = self.remotes_config.write().unwrap();
        *writer = remotes_config;
    }

    fn read_remotes_config(&self) {
        let location = self.get_remotes_config_location();
        match std::fs::read_to_string(&location) {
            Ok(toml_string) => match toml::from_str(&toml_string) {
                Ok(remotes_config) => {
                    self.set_remotes_config(remotes_config);
                }
                Err(e) => {
                    error!("Failed to load the remotes configuration from {}: Invalid TOML: {}", location.to_str().unwrap_or(""), e);
                }
            },
            Err(e) => {
                error!("Failed to load the remotes configuration from {}: {}", location.to_str().unwrap_or(""), e);
            }
        }
    }

    fn write_remotes_config(&self) {
        let location = self.get_remotes_config_location();
        let remotes_config = self.get_remotes_config();
        match toml::to_string(&remotes_config) {
            Ok(toml_string) => match std::fs::write(location.clone(), toml_string) {
                Ok(_) => info!("Saved remote configuration to {}", location.to_str().unwrap_or("")),
                Err(e) => error!("Failed to save remote configuration to {}: {}", location.to_str().unwrap_or(""), e),
            },
            Err(e) => error!("Failed to save remote configuration to {}: {}", location.to_str().unwrap_or(""), e),
        }
    }
}

#[async_trait]
impl Lifecycle for ConfigManagerImpl {
    async fn init(&self) {
        self.read_graphql_server_config();
        self.read_instance_config();
        self.read_plugins_config();
        self.read_remotes_config();
    }
}
