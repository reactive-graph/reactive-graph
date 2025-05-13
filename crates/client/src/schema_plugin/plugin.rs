#[derive(Clone, Debug, cynic::QueryFragment)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-plugin-schema.graphql",
    schema_module = "crate::schema_plugin::schema"
)]
pub struct Plugin {
    pub name: String,
    pub short_name: String,
    pub state: String,
    pub description: String,
    pub path: String,
    pub stem: String,
    pub version: String,
    pub plugin_api_version: String,
    pub rustc_version: String,
}

#[derive(Clone, Debug, cynic::QueryFragment)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-plugin-schema.graphql",
    schema_module = "crate::schema_plugin::schema",
    graphql_type = "Plugin"
)]
pub struct PluginDependencies {
    pub dependencies: Vec<Plugin>,
}

#[derive(Clone, Debug, cynic::QueryFragment)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-plugin-schema.graphql",
    schema_module = "crate::schema_plugin::schema",
    graphql_type = "Plugin"
)]
pub struct PluginDependents {
    pub dependents: Vec<Plugin>,
}

#[derive(Clone, Debug, cynic::QueryFragment)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-plugin-schema.graphql",
    schema_module = "crate::schema_plugin::schema",
    graphql_type = "Plugin"
)]
pub struct PluginUnsatisfiedDependencies {
    pub unsatisfied_dependencies: Vec<Plugin>,
}

impl From<&Plugin> for reactive_graph_plugin_model::Plugin {
    fn from(plugin: &Plugin) -> Self {
        reactive_graph_plugin_model::Plugin {
            name: plugin.name.clone(),
            short_name: plugin.short_name.clone(),
            state: plugin.state.clone(),
            version: plugin.version.clone(),
            plugin_api_version: plugin.plugin_api_version.clone(),
            rustc_version: plugin.rustc_version.clone(),
        }
    }
}
