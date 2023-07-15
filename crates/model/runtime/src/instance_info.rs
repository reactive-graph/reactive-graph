use crate::config::InstanceAddress;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceInfo {
    /// The name of the instance.
    pub name: String,

    /// A description text about the instance.
    pub description: String,

    /// The hostname.
    pub hostname: String,

    /// The port.
    pub port: u16,

    /// Secure endpoint.
    pub secure: bool,

    /// The version of the runtime (version field in Cargo.toml).
    pub version: String,

    /// The build date of the runtime.
    pub build_date: String,

    /// The git branch.
    pub git_branch: String,

    /// The git commit.
    pub git_commit: String,

    /// The rust compiler version.
    pub rustc_version: String,

    /// The plugin api version.
    pub plugin_api_version: String,

    /// When the remote instance was last seen.
    pub last_seen: DateTime<Utc>,
}

impl InstanceInfo {
    pub fn address(&self) -> InstanceAddress {
        InstanceAddress::new(self.hostname.clone(), self.port, self.secure)
    }
}

impl PartialEq for InstanceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.hostname == other.hostname && self.port == other.port && self.secure == other.secure
    }
}

impl PartialEq<InstanceAddress> for InstanceInfo {
    fn eq(&self, other: &InstanceAddress) -> bool {
        self.hostname == other.hostname && self.port == other.port && self.secure == other.secure
    }
}

impl From<InstanceInfo> for InstanceAddress {
    fn from(instance_info: InstanceInfo) -> Self {
        InstanceAddress::new(instance_info.hostname, instance_info.port, instance_info.secure)
    }
}
