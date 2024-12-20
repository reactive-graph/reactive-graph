use crate::InstanceAddress;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Clone, Debug, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceInfo {
    /// The name of the instance.
    pub name: String,

    /// A description text about the instance.
    pub description: String,

    /// The instance address.
    #[serde(flatten)]
    pub address: InstanceAddress,

    /// The version of the runtime (version field in Cargo.toml).
    pub version: String,

    /// The git commit.
    pub git_commit: String,

    /// The git tag.
    pub git_tag: String,

    /// The rust compiler channel.
    pub rustc_channel: String,

    /// The rust compiler version.
    pub rustc_version: String,

    /// The plugin api version.
    pub plugin_api_version: String,

    /// When the remote instance was last seen.
    pub last_seen: DateTime<Utc>,
}

impl InstanceInfo {
    pub fn address(&self) -> InstanceAddress {
        self.address.clone()
    }
}

// An InstanceInfo is equal if the InstanceAddress is equal
impl PartialEq for InstanceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl PartialEq<InstanceAddress> for InstanceInfo {
    fn eq(&self, other: &InstanceAddress) -> bool {
        &self.address == other
    }
}

impl From<InstanceInfo> for InstanceAddress {
    fn from(instance_info: InstanceInfo) -> Self {
        instance_info.address.clone()
    }
}
