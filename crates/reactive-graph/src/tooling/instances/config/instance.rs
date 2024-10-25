use crate::shared::config::instance::InstanceConfigArgs;
use crate::tooling::instances::config::serde::read_config_file;
use crate::tooling::instances::config::serde::write_config_file;
use reactive_graph_config_model::InstanceConfig;
use std::path::PathBuf;

pub const INSTANCE_CONFIG_FILENAME: &str = "instance.toml";

pub fn handle_instance_config(config_dir: &PathBuf, args: InstanceConfigArgs) -> anyhow::Result<()> {
    let path = get_instance_config_path(config_dir);
    let mut config: InstanceConfig = read_config_file(&path)?;
    let mut changed = false;
    if let Some(name) = args.name {
        config.name = name;
        changed = true;
    }
    if let Some(description) = args.description {
        config.description = description;
        changed = true;
    }
    if changed {
        write_config_file(&path, config)?;
    }

    Ok(())
}

pub fn get_instance_config_path(config_dir: &PathBuf) -> PathBuf {
    let mut path = config_dir.to_owned();
    path.push(INSTANCE_CONFIG_FILENAME);
    path
}
