use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fs;
use std::fs::read_to_string;
use std::path::PathBuf;

pub fn read_config_file<T: DeserializeOwned>(path: &PathBuf) -> anyhow::Result<T> {
    let contents = read_to_string(path)?;
    Ok(toml::from_str(&contents)?)
}

pub fn write_config_file<T: Serialize>(path: &PathBuf, config: T) -> anyhow::Result<()> {
    let contents = toml::to_string_pretty(&config)?;
    Ok(fs::write(path, contents)?)
}
