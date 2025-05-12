use std::ops::Deref;

use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;

use reactive_graph_remotes_model::InstanceAddress;

#[derive(Clone, Debug, cynic::QueryFragment)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-runtime-schema.graphql",
    schema_module = "crate::schema_runtime::schema"
)]
pub struct InstanceInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub plugin_api_version: String,
    pub rustc_channel: String,
    pub rustc_version: String,
    pub hostname: String,
    pub port: i32,
    pub secure: bool,
    pub git_commit: String,
    pub git_tag: String,
    pub last_seen: String,
}

impl From<InstanceInfo> for reactive_graph_remotes_model::InstanceInfo {
    fn from(instance_info: InstanceInfo) -> Self {
        let last_seen = DateTime::<FixedOffset>::parse_from_rfc3339(&instance_info.last_seen)
            .map(|result| result.into())
            .unwrap_or(Utc::now());
        reactive_graph_remotes_model::InstanceInfo {
            name: instance_info.name,
            description: instance_info.description,
            version: instance_info.version,
            plugin_api_version: instance_info.plugin_api_version,
            rustc_channel: instance_info.rustc_channel,
            rustc_version: instance_info.rustc_version,
            address: InstanceAddress::new(instance_info.hostname, u16::try_from(instance_info.port).unwrap_or(0), instance_info.secure),
            git_commit: instance_info.git_commit,
            git_tag: instance_info.git_tag,
            last_seen,
        }
    }
}

pub struct InstanceInfos(pub Vec<InstanceInfo>);

impl Deref for InstanceInfos {
    type Target = Vec<InstanceInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<InstanceInfos> for Vec<reactive_graph_remotes_model::InstanceInfo> {
    fn from(instances_info: InstanceInfos) -> Self {
        instances_info.0.into_iter().map(From::from).collect()
    }
}
