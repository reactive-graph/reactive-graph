#[cynic::schema_for_derives(file = r#"schema_plugin.graphql"#, module = "crate::schema_plugin::schema")]
pub mod queries {
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariables;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariablesFields;
    use crate::Plugin;
    use crate::PluginUnsatisfiedDependencies;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "PluginByNameVariables")]
    pub struct GetUnsatisfiedDependencies {
        #[arguments(name: $name)]
        pub plugins: Vec<PluginUnsatisfiedDependencies>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Plugin", variables = "PluginByNameVariables")]
    pub struct GetUnsatisfiedDependenciesPlugin {
        pub unsatisfied_dependencies: Vec<Plugin>,
    }

    pub fn get_unsatisfied_dependencies(name: String) -> cynic::Operation<GetUnsatisfiedDependencies, PluginByNameVariables> {
        use cynic::QueryBuilder;
        GetUnsatisfiedDependencies::build(name.into())
    }
}
