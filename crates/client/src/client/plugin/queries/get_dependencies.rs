#[cynic::schema_for_derives(file = r#"schema_plugin.graphql"#, module = "crate::schema_plugin::schema")]
pub mod queries {
    use crate::Plugin;
    use crate::PluginDependencies;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariables;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "PluginByNameVariables")]
    pub struct GetDependencies {
        #[arguments(name: $name)]
        pub plugins: Vec<PluginDependencies>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Plugin", variables = "PluginByNameVariables")]
    pub struct GetDependenciesPlugin {
        pub dependencies: Vec<Plugin>,
    }

    pub fn get_dependencies(name: String) -> cynic::Operation<GetDependencies, PluginByNameVariables> {
        use cynic::QueryBuilder;
        GetDependencies::build(name.into())
    }
}
