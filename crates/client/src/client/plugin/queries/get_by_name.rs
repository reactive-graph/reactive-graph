#[cynic::schema_for_derives(file = r#"schema_plugin.graphql"#, module = "crate::schema_plugin::schema")]
pub mod queries {
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariables;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariablesFields;
    use crate::Plugin;

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
