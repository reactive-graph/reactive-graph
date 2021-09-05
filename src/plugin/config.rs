use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginsConfig {
    pub plugin: Vec<PluginConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginConfig {
    pub name: String,
    pub active: bool,
    pub path: String,
}
