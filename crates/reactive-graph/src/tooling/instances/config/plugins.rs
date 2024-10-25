use crate::shared::config::plugins::PluginsConfigArgs;
use crate::tooling::instances::config::serde::read_config_file;
use crate::tooling::instances::config::serde::write_config_file;
use reactive_graph_config_model::PluginsConfig;
use std::path::PathBuf;

pub const PLUGINS_CONFIG_FILENAME: &str = "plugins.toml";

pub fn handle_plugins_config(config_dir: &PathBuf, args: PluginsConfigArgs) -> anyhow::Result<()> {
    let path = get_plugins_config_path(config_dir);
    let mut config: PluginsConfig = read_config_file(&path)?;
    let mut changed = false;
    if let Some(disable_all_plugins) = args.disable_all_plugins {
        config.disabled = Some(disable_all_plugins);
        changed = true;
    }
    if let Some(disabled_plugins) = args.disabled_plugins {
        config.disabled_plugins = Some(disabled_plugins);
        changed = true;
    }
    if let Some(enabled_plugins) = args.enabled_plugins {
        config.enabled_plugins = Some(enabled_plugins);
        changed = true;
    }
    if let Some(disable_hot_deploy) = args.disable_hot_deploy {
        config.hot_deploy = Some(!disable_hot_deploy);
        changed = true;
    }
    if let Some(install_location) = args.install_location {
        config.install_location = Some(install_location);
        changed = true;
    }
    if let Some(hot_deploy_location) = args.hot_deploy_location {
        config.hot_deploy_location = Some(hot_deploy_location);
        changed = true;
    }
    if changed {
        write_config_file(&path, config)?;
    }
    Ok(())
}

pub fn get_plugins_config_path(config_dir: &PathBuf) -> PathBuf {
    let mut path = config_dir.to_owned();
    path.push(PLUGINS_CONFIG_FILENAME);
    path
}
