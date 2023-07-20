use tabled::Tabled;

use crate::table_model::container::DefaultTableContainer;
use crate::table_model::container::DefaultTableOptions;

#[derive(Clone, Debug, Tabled)]
pub struct InstanceInfo {
    pub name: String,
    #[tabled(skip)]
    pub description: String,
    pub version: String,
    pub plugin_api_version: String,
    pub rustc_version: String,
    pub hostname: String,
    pub port: i32,
    #[tabled(skip)]
    pub secure: bool,
    #[tabled(skip)]
    pub git_branch: String,
    #[tabled(skip)]
    pub git_commit: String,
    #[tabled(skip)]
    pub build_date: String,
    #[tabled(skip)]
    pub last_seen: String,
}

impl From<crate::model_runtime::InstanceInfo> for InstanceInfo {
    fn from(instance_info: crate::model_runtime::InstanceInfo) -> Self {
        InstanceInfo {
            name: instance_info.name,
            description: instance_info.description,
            version: instance_info.version,
            plugin_api_version: instance_info.plugin_api_version,
            rustc_version: instance_info.rustc_version,
            hostname: instance_info.address.hostname,
            port: i32::from(instance_info.address.port),
            secure: instance_info.address.secure,
            git_branch: instance_info.git_branch,
            git_commit: instance_info.git_commit,
            build_date: instance_info.build_date,
            last_seen: instance_info.last_seen.to_rfc3339(),
        }
    }
}

pub(crate) type InstanceInfos = DefaultTableContainer<crate::model_runtime::InstanceInfo, InstanceInfo, DefaultTableOptions>;
