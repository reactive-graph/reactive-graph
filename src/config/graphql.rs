use actix_web::middleware::Logger;
use log::error;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct GraphQLLoggingConfig {
    pub enabled: bool,
    pub format: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GraphQLServerConfig {
    pub hostname: String,
    pub port: i32,
    pub secure: Option<bool>,
    pub shutdown_timeout: Option<u64>,
    pub workers: Option<usize>,
    pub default_base_path: Option<String>,
    pub logging: Option<GraphQLLoggingConfig>,
}

impl Default for GraphQLServerConfig {
    fn default() -> Self {
        GraphQLServerConfig {
            hostname: String::from("localhost"),
            port: 31415,
            secure: None,
            shutdown_timeout: None,
            workers: None,
            default_base_path: None,
            logging: None,
        }
    }
}

impl ToString for GraphQLServerConfig {
    fn to_string(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}

pub(crate) fn get_graphql_server_config() -> GraphQLServerConfig {
    // TODO: resolve config file from CONFIG_LOCATION(s)
    match std::fs::read_to_string("./config/graphql.toml") {
        Ok(toml_string) => {
            let graphql_server_config: Result<GraphQLServerConfig, _> = toml::from_str(&toml_string);
            if graphql_server_config.is_err() {
                error!("Failed to load graphql configuration from {}: Invalid TOML", "./config/graphql.toml");
            }
            graphql_server_config.unwrap_or_default()
        }
        Err(_) => {
            error!("Failed to load graphql configuration from {}: File does not exist", "./config/graphql.toml");
            GraphQLServerConfig::default()
        }
    }
}

pub(crate) fn get_logger_middleware() -> Option<Logger> {
    let graphql_server_config = crate::config::graphql::get_graphql_server_config();
    match graphql_server_config.logging {
        Some(logging_config) => {
            if logging_config.enabled {
                match logging_config.format {
                    Some(format) => Some(Logger::new(format.as_str())),
                    None => Some(Logger::default()),
                }
            } else {
                None
            }
            // logger
        }
        None => None,
    }
}
