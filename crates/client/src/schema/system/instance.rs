use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use std::ops::Deref;

#[derive(Clone, Debug, cynic::QueryFragment)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
pub struct InstanceInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub plugin_api_version: String,
    pub rustc_version: String,
    pub hostname: String,
    pub port: i32,
    pub secure: bool,
    pub git_branch: String,
    pub git_commit: String,
    pub build_date: String,
    pub last_seen: String,
}

impl From<InstanceInfo> for crate::model_runtime::InstanceInfo {
    fn from(instance_info: InstanceInfo) -> Self {
        let last_seen = DateTime::<FixedOffset>::parse_from_rfc3339(&instance_info.last_seen)
            .map(|result| result.into())
            .unwrap_or(Utc::now());
        crate::model_runtime::InstanceInfo {
            name: instance_info.name,
            description: instance_info.description,
            version: instance_info.version,
            plugin_api_version: instance_info.plugin_api_version,
            rustc_version: instance_info.rustc_version,
            hostname: instance_info.hostname,
            port: u16::try_from(instance_info.port).unwrap_or(0),
            secure: instance_info.secure,
            git_branch: instance_info.git_branch,
            git_commit: instance_info.git_commit,
            build_date: instance_info.build_date,
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

impl From<InstanceInfos> for Vec<crate::model_runtime::InstanceInfo> {
    fn from(instances_info: InstanceInfos) -> Self {
        instances_info.0.into_iter().map(From::from).collect()
    }
}
