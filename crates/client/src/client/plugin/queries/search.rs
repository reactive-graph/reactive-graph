#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-plugin-schema.graphql"#, module = "crate::schema_plugin::schema")]
pub mod queries {
    use crate::Plugin;
    use crate::client::plugin::variables::search::variables::SearchPluginVariables;
    use crate::client::plugin::variables::search::variables::SearchPluginVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "SearchPluginVariables")]
    pub struct SearchPlugins {
        #[arguments(name: $name, state: $state, stem: $stem)]
        pub plugins: Vec<Plugin>,
    }

    pub fn search(vars: SearchPluginVariables) -> cynic::Operation<SearchPlugins, SearchPluginVariables> {
        use cynic::QueryBuilder;
        SearchPlugins::build(vars)
    }
}
