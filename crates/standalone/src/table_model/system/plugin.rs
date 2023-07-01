use tabled::Table;
use tabled::Tabled;

#[derive(Tabled)]
pub(crate) struct Plugin {
    pub name: String,
    pub short_name: String,
    pub state: String,
    pub version: String,
    pub plugin_api_version: String,
    pub rustc_version: String,
}

#[derive(Tabled)]
pub(crate) struct PluginDependencies {
    #[tabled(display_with("display_plugins"))]
    pub dependencies: Vec<Plugin>,
}

#[derive(Tabled)]
pub(crate) struct PluginDependents {
    #[tabled(display_with("display_plugins"))]
    pub dependents: Vec<Plugin>,
}

pub(crate) fn display_plugins(plugins: &Vec<Plugin>) -> String {
    Table::new(plugins).to_string()
}

impl From<crate::client::Plugin> for Plugin {
    fn from(plugin: inexor_rgf_client::Plugin) -> Self {
        Plugin {
            name: plugin.name,
            short_name: plugin.short_name,
            state: plugin.state,
            version: plugin.version,
            plugin_api_version: plugin.plugin_api_version,
            rustc_version: plugin.rustc_version,
        }
    }
}
