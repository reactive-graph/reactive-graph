#[cynic::schema_for_derives(file = r#"schema_plugin.graphql"#, module = "crate::schema_plugin::schema")]
pub mod queries {
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariables;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariablesFields;
    use crate::Plugin;
    use crate::PluginDependents;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "PluginByNameVariables")]
    pub struct GetDependents {
        #[arguments(name: $name)]
        pub plugins: Vec<PluginDependents>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Plugin", variables = "PluginByNameVariables")]
    pub struct GetDependentsPlugin {
        pub dependents: Vec<Plugin>,
    }

    pub fn get_dependents(name: String) -> cynic::Operation<GetDependents, PluginByNameVariables> {
        use cynic::QueryBuilder;
        GetDependents::build(name.into())
    }
}
