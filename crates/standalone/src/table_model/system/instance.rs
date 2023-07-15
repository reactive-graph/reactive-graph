use tabled::settings::Style;
use tabled::Table;
use tabled::Tabled;

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

pub(crate) struct InstanceInfos(pub(crate) Vec<InstanceInfo>);

impl InstanceInfos {
    pub fn table(&self) -> Table {
        Table::new(self.0.to_vec().iter()).with(Style::extended()).to_owned()
    }
}

impl From<Vec<crate::model_runtime::InstanceInfo>> for InstanceInfos {
    fn from(instance_info: Vec<crate::model_runtime::InstanceInfo>) -> Self {
        InstanceInfos(instance_info.into_iter().map(From::from).collect())
    }
}

impl From<InstanceInfo> for InstanceInfos {
    fn from(instance_info: InstanceInfo) -> Self {
        InstanceInfos(vec![instance_info])
    }
}

impl From<crate::model_runtime::InstanceInfo> for InstanceInfos {
    fn from(instance_info: crate::model_runtime::InstanceInfo) -> Self {
        InstanceInfos(vec![instance_info.into()])
    }
}

impl ToString for InstanceInfos {
    fn to_string(&self) -> String {
        self.table().to_string()
    }
}
