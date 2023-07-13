#[derive(Clone, Debug, cynic::QueryFragment)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
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
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema", graphql_type = "Plugin")]
pub struct PluginDependencies {
    pub dependencies: Vec<Plugin>,
}

#[derive(Clone, Debug, cynic::QueryFragment)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema", graphql_type = "Plugin")]
pub struct PluginDependents {
    pub dependents: Vec<Plugin>,
}
