use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use springtime_di::component_alias;
use springtime_di::Component;

use reactive_graph_config_api::ConfigManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_api::PLUGIN_API_VERSION;
use reactive_graph_remotes_api::InstanceService;
use reactive_graph_remotes_model::InstanceInfo;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static TARGET_TRIPLE: &str = env!("VERGEN_CARGO_TARGET_TRIPLE");
pub static GIT_TAG: &str = env!("VERGEN_GIT_DESCRIBE");
pub static GIT_COMMIT: &str = env!("VERGEN_GIT_SHA");
pub static RUSTC_CHANNEL: &str = env!("VERGEN_RUSTC_CHANNEL");
pub static RUSTC_VERSION: &str = env!("VERGEN_RUSTC_SEMVER");

#[derive(Component)]
pub struct InstanceServiceImpl {
    config_manager: Arc<dyn ConfigManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl InstanceService for InstanceServiceImpl {
    fn get_instance_info(&self) -> InstanceInfo {
        let instance_config = self.config_manager.get_instance_config();
        let graphql_server_config = self.config_manager.get_graphql_server_config();
        InstanceInfo {
            name: instance_config.name,
            description: instance_config.description,
            address: graphql_server_config.address(),
            version: String::from(VERSION),
            git_commit: String::from(GIT_COMMIT),
            git_tag: String::from(GIT_TAG),
            rustc_channel: String::from(RUSTC_CHANNEL),
            rustc_version: String::from(RUSTC_VERSION),
            plugin_api_version: String::from(PLUGIN_API_VERSION),
            last_seen: Utc::now(),
        }
    }
}

#[async_trait]
impl Lifecycle for InstanceServiceImpl {}
