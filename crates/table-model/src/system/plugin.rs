use tabled::Table;
use tabled::Tabled;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::settings::object::Rows;
use tabled::settings::peaker::Priority;

use crate::container::DefaultTableContainer;
use crate::container::TableInlineFormat;
use crate::container::TableInlineFormatSetter;
use crate::container::TableOptions;

#[derive(Clone, Debug, Tabled)]
pub struct Plugin {
    pub name: String,
    pub short_name: String,
    pub state: String,
    pub version: String,
    pub plugin_api_version: String,
    pub rustc_version: String,

    #[tabled(skip)]
    inline_format: TableInlineFormat,
}

#[allow(unused)]
#[derive(Clone, Debug, Tabled)]
pub(crate) struct PluginDependencies {
    #[tabled(display_with("display_plugins"))]
    pub dependencies: Vec<Plugin>,
}

#[allow(unused)]
#[derive(Clone, Debug, Tabled)]
pub(crate) struct PluginDependents {
    #[tabled(display_with("display_plugins"))]
    pub dependents: Vec<Plugin>,
}

#[allow(unused)]
pub(crate) fn display_plugins(plugins: &Vec<Plugin>) -> String {
    Table::new(plugins).to_string()
}

impl TableInlineFormatSetter for Plugin {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
}

impl From<reactive_graph_plugin_model::Plugin> for Plugin {
    fn from(plugin: reactive_graph_plugin_model::Plugin) -> Self {
        Plugin {
            name: plugin.name,
            short_name: plugin.short_name,
            state: plugin.state,
            version: plugin.version,
            plugin_api_version: plugin.plugin_api_version,
            rustc_version: plugin.rustc_version,
            inline_format: Default::default(),
        }
    }
}

pub type PluginsTableContainer = DefaultTableContainer<reactive_graph_plugin_model::Plugin, Plugin, PluginsTableOptions>;

pub struct PluginsTableOptions;

impl TableOptions for PluginsTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended()).with(
            Modify::new(Rows::new(1..))
                .with(Width::increase(10))
                .with(Width::truncate(40).suffix("...").priority(Priority::max(false))),
        )
    }
}
