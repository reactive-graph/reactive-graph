use log::error;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct GraphSqlServerConfig {
    pub hostname: String,
    pub port: i32,
    pub shutdown_timeout: Option<u64>,
    pub workers: Option<usize>,
    pub default_base_path: Option<String>,
}

impl Default for GraphSqlServerConfig {
    fn default() -> Self {
        GraphSqlServerConfig {
            hostname: String::from("localhost"),
            port: 31415,
            shutdown_timeout: None,
            workers: None,
            default_base_path: None,
        }
    }
}

impl ToString for GraphSqlServerConfig {
    fn to_string(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}

pub(crate) fn get_graphql_server_config() -> GraphSqlServerConfig {
    // TODO: resolve config file from CONFIG_LOCATION(s)
    match std::fs::read_to_string("./config/graphql.toml") {
        Ok(toml_string) => {
            let graphql_server_config: Result<GraphSqlServerConfig, _> = toml::from_str(&toml_string);
            if graphql_server_config.is_err() {
                error!("Failed to load graphql configuration from {}: Invalid TOML", "./config/graphql.toml");
            }
            graphql_server_config.unwrap_or_default()
        }
        Err(_) => {
            error!("Failed to load graphql configuration from {}: File does not exist", "./config/graphql.toml");
            GraphSqlServerConfig::default()
        }
    }
}
