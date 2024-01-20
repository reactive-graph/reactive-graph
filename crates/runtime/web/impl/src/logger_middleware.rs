use actix_web::middleware::Logger;

use inexor_rgf_config_model::GraphQLLoggingConfig;

pub fn get_logger_middleware(graphql_logging_config: &GraphQLLoggingConfig) -> Option<Logger> {
    if graphql_logging_config.enabled {
        match &graphql_logging_config.format {
            Some(format) => Some(Logger::new(format.as_str())),
            None => Some(Logger::default()),
        }
    } else {
        None
    }
}
