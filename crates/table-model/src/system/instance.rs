use tabled::Tabled;

use crate::container::DefaultTableContainer;
use crate::container::DefaultTableOptions;
use crate::container::TableInlineFormat;
use crate::container::TableInlineFormatSetter;

#[derive(Clone, Debug, Tabled)]
pub struct InstanceInfo {
    pub name: String,
    #[allow(unused)]
    #[tabled(skip)]
    pub description: String,
    pub version: String,
    pub plugin_api_version: String,
    pub rustc_channel: String,
    pub rustc_version: String,
    pub hostname: String,
    pub port: i32,
    #[allow(unused)]
    #[tabled(skip)]
    pub secure: bool,
    #[allow(unused)]
    #[tabled(skip)]
    pub git_commit: String,
    #[allow(unused)]
    #[tabled(skip)]
    pub git_tag: String,
    #[allow(unused)]
    #[tabled(skip)]
    pub last_seen: String,

    #[tabled(skip)]
    inline_format: TableInlineFormat,
}

impl TableInlineFormatSetter for InstanceInfo {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
}

impl From<reactive_graph_remotes_model::InstanceInfo> for InstanceInfo {
    fn from(instance_info: reactive_graph_remotes_model::InstanceInfo) -> Self {
        InstanceInfo {
            name: instance_info.name,
            description: instance_info.description,
            version: instance_info.version,
            plugin_api_version: instance_info.plugin_api_version,
            rustc_channel: instance_info.rustc_channel,
            rustc_version: instance_info.rustc_version,
            hostname: instance_info.address.hostname,
            port: i32::from(instance_info.address.port),
            secure: instance_info.address.secure,
            git_commit: instance_info.git_commit,
            git_tag: instance_info.git_tag,
            last_seen: instance_info.last_seen.to_rfc3339(),
            inline_format: Default::default(),
        }
    }
}

pub type InstanceInfos = DefaultTableContainer<reactive_graph_remotes_model::InstanceInfo, InstanceInfo, DefaultTableOptions>;
