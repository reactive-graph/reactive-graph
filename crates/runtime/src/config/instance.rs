use log::error;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct InstanceConfig {
    /// The name of the instance.
    pub name: String,

    /// A description text about the instance.
    pub description: String,
}

impl Default for InstanceConfig {
    fn default() -> Self {
        InstanceConfig {
            name: String::from("Default"),
            description: String::from("This is the default instance."),
        }
    }
}

pub(crate) fn get_instance_config() -> InstanceConfig {
    match std::fs::read_to_string("../../../../config/instance.toml") {
        Ok(toml_string) => {
            let instance_config: Result<InstanceConfig, _> = toml::from_str(&toml_string);
            if instance_config.is_err() {
                error!("Failed to load instance configuration from {}: Invalid TOML", "./config/instance.toml");
            }
            instance_config.unwrap_or_default()
        }
        Err(_) => {
            error!("Failed to load instance configuration from {}: File does not exist", "./config/instance.toml");
            InstanceConfig::default()
        }
    }
}
