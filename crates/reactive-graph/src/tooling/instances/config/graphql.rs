use crate::shared::config::graphql::GraphQLServerConfigArgs;
use crate::tooling::instances::config::serde::read_config_file;
use crate::tooling::instances::config::serde::write_config_file;
use reactive_graph_config_model::GraphQLServerConfig;
use std::path::PathBuf;

pub const GRAPHQL_CONFIG_FILENAME: &str = "graphql.toml";

pub fn handle_graphql_config(config_dir: &PathBuf, args: GraphQLServerConfigArgs) -> anyhow::Result<()> {
    let path = get_graphql_config_path(config_dir);
    let mut config: GraphQLServerConfig = read_config_file(&path)?;
    let mut changed = false;
    if let Some(hostname) = args.hostname {
        config.hostname = Some(hostname);
        changed = true;
    }
    if let Some(port) = args.port {
        config.port = Some(port);
        changed = true;
    }
    if let Some(secure) = args.secure {
        config.secure = Some(secure);
        changed = true;
    }
    if let Some(default_context_path) = args.default_context_path {
        config.default_context_path = Some(default_context_path);
        changed = true;
    }
    if let Some(shutdown_timeout) = args.shutdown_timeout {
        config.shutdown_timeout = Some(shutdown_timeout);
        changed = true;
    }
    if let Some(ssl_certificate_path) = args.ssl_certificate_path {
        config.ssl_certificate_path = Some(ssl_certificate_path);
        changed = true;
    }
    if let Some(ssl_private_key_path) = args.ssl_private_key_path {
        config.ssl_private_key_path = Some(ssl_private_key_path);
        changed = true;
    }
    if let Some(workers) = args.workers {
        config.workers = Some(workers);
        changed = true;
    }
    if changed {
        write_config_file(&path, config)?;
    }

    Ok(())
}

pub fn get_graphql_config_path(config_dir: &PathBuf) -> PathBuf {
    let mut path = config_dir.to_owned();
    path.push(GRAPHQL_CONFIG_FILENAME);
    path
}
