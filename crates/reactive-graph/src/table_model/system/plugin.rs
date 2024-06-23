use tabled::settings::object::Rows;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::Table;
use tabled::Tabled;

use crate::table_model::container::DefaultTableContainer;
use crate::table_model::container::TableOptions;

#[derive(Clone, Debug, Tabled)]
pub(crate) struct Plugin {
    pub name: String,
    pub short_name: String,
    pub state: String,
    pub version: String,
    pub plugin_api_version: String,
    pub rustc_version: String,
}

#[derive(Clone, Debug, Tabled)]
pub(crate) struct PluginDependencies {
    #[tabled(display_with("display_plugins"))]
    pub dependencies: Vec<Plugin>,
}

#[derive(Clone, Debug, Tabled)]
pub(crate) struct PluginDependents {
    #[tabled(display_with("display_plugins"))]
    pub dependents: Vec<Plugin>,
}

pub(crate) fn display_plugins(plugins: &Vec<Plugin>) -> String {
    Table::new(plugins).to_string()
}

impl From<reactive_graph_client::Plugin> for Plugin {
    fn from(plugin: reactive_graph_client::Plugin) -> Self {
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

pub(crate) type Plugins = DefaultTableContainer<reactive_graph_client::Plugin, Plugin, PluginsTableOptions>;

pub(crate) struct PluginsTableOptions;

impl TableOptions for PluginsTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended()).with(
            Modify::new(Rows::new(1..))
                .with(Width::increase(10).priority())
                .with(Width::truncate(40).suffix("...")),
        )
    }
}
