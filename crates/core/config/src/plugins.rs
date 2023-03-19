use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

const DEFAULT_HOT_DEPLOY_LOCATION: &str = "./plugins/deploy";

const DEFAULT_INSTALL_LOCATION: &str = "./plugins/installed";

/// Configuration of the plugin system.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginsConfig {
    /// If true, the plugin system is disabled.
    pub disabled: Option<bool>,

    /// The plugins which are disabled.
    pub disabled_plugins: Option<Vec<String>>,

    /// If true, hot deployment is enabled.
    pub hot_deploy: Option<bool>,

    /// The folder which is watched for hot deployment.
    pub hot_deploy_location: Option<String>,

    /// The folder which plugins are installed permanently.
    pub install_location: Option<String>,
}

impl PluginsConfig {
    pub fn is_hot_deploy(&self) -> bool {
        self.hot_deploy.unwrap_or(true)
    }

    pub fn get_hot_deploy_location(&self) -> Option<PathBuf> {
        if !self.is_hot_deploy() {
            return None;
        }
        fs::canonicalize(PathBuf::from(self.hot_deploy_location.clone().unwrap_or(DEFAULT_HOT_DEPLOY_LOCATION.to_string()))).ok()
    }

    pub fn get_install_location(&self) -> Option<PathBuf> {
        fs::canonicalize(PathBuf::from(self.install_location.clone().unwrap_or(DEFAULT_INSTALL_LOCATION.to_string()))).ok()
    }
}

impl Default for PluginsConfig {
    fn default() -> Self {
        PluginsConfig {
            disabled: Some(false),
            disabled_plugins: Some(Vec::new()),
            hot_deploy: Some(true),
            hot_deploy_location: Some(DEFAULT_HOT_DEPLOY_LOCATION.to_string()),
            install_location: Some(DEFAULT_INSTALL_LOCATION.to_string()),
        }
    }
}
