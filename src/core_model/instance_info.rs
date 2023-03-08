use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct InstanceInfo {
    /// The name of the instance.
    pub name: String,

    /// A description text about the instance.
    pub description: String,

    /// The hostname.
    pub hostname: String,

    /// The port.
    pub port: i32,

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
}
