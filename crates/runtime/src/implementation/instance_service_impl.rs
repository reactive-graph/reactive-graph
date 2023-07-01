use async_trait::async_trait;
use chrono::Utc;

use crate::api::ConfigManager;
use crate::api::InstanceService;
use crate::api::Lifecycle;
use crate::di::*;
use crate::model_runtime::InstanceInfo;
use crate::plugins::PLUGIN_API_VERSION;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static BUILD_DATE: &str = env!("VERGEN_BUILD_DATE");
pub static GIT_BRANCH: &str = env!("VERGEN_GIT_BRANCH");
pub static GIT_COMMIT: &str = env!("VERGEN_GIT_SHA");
pub static RUSTC_VERSION: &str = env!("VERGEN_RUSTC_SEMVER");

#[component]
pub struct InstanceServiceImpl {
    config_manager: Wrc<dyn ConfigManager>,
}

#[async_trait]
#[provides]
impl InstanceService for InstanceServiceImpl {
    fn get_instance_info(&self) -> InstanceInfo {
        let instance_config = self.config_manager.get_instance_config();
        let graphql_server_config = self.config_manager.get_graphql_server_config();
        InstanceInfo {
            name: instance_config.name,
            description: instance_config.description,
            hostname: graphql_server_config.hostname(),
            port: graphql_server_config.port(),
            secure: graphql_server_config.is_secure(),
            version: String::from(VERSION),
            build_date: String::from(BUILD_DATE),
            git_branch: String::from(GIT_BRANCH),
            git_commit: String::from(GIT_COMMIT),
            rustc_version: String::from(RUSTC_VERSION),
            plugin_api_version: String::from(PLUGIN_API_VERSION),
            last_seen: Utc::now(),
        }
    }
}

#[async_trait]
impl Lifecycle for InstanceServiceImpl {}
