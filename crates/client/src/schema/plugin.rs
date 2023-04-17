use tabled::Table;
use tabled::Tabled;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
#[derive(Tabled)]
pub struct Plugin {
    pub name: String,
    pub short_name: String,
    pub state: String,
    #[tabled(skip)]
    pub description: String,
    #[tabled(skip)]
    pub path: String,
    #[tabled(skip)]
    pub stem: String,
    pub version: String,
    pub plugin_api_version: String,
    pub rustc_version: String,
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema", graphql_type = "Plugin")]
#[derive(Tabled)]
pub struct PluginDependencies {
    #[tabled(display_with("display_plugins"))]
    pub dependencies: Vec<Plugin>,
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema", graphql_type = "Plugin")]
#[derive(Tabled)]
pub struct PluginDependents {
    #[tabled(display_with("display_plugins"))]
    pub dependents: Vec<Plugin>,
}

pub fn display_plugins(plugins: &Vec<Plugin>) -> String {
    Table::new(plugins).to_string()
}
