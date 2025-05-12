#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-plugin-schema.graphql"#, module = "crate::schema_plugin::schema")]
pub mod mutations {
    use crate::Plugin;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariables;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariablesFields;

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
