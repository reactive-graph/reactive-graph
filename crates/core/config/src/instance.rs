use serde::Deserialize;
use serde::Serialize;

/// Configuration of the runtime instance.
#[derive(Debug, Clone, Deserialize, Serialize)]
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
