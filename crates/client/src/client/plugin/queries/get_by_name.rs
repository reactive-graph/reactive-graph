#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-plugin-schema.graphql"#, module = "crate::schema_plugin::schema")]
pub mod queries {
    use crate::Plugin;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariables;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "PluginByNameVariables")]
    pub struct GetPluginByName {
        #[arguments(name: $name)]
        pub plugins: Vec<Plugin>,
    }

    pub fn get_by_name(name: String) -> cynic::Operation<GetPluginByName, PluginByNameVariables> {
        use cynic::QueryBuilder;
        let vars: PluginByNameVariables = name.into();
        GetPluginByName::build(vars)
    }
}
