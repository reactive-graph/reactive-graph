#[cynic::schema_for_derives(file = r#"schema_plugin.graphql"#, module = "crate::schema_plugin::schema")]
pub mod mutations {
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariables;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariablesFields;
    use crate::Plugin;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PluginByNameVariables")]
    pub struct StopPlugin {
        #[arguments(name: $name)]
        pub stop: Plugin,
    }

    pub fn stop(name: String) -> cynic::Operation<StopPlugin, PluginByNameVariables> {
        use cynic::MutationBuilder;
        StopPlugin::build(name.into())
    }
}
