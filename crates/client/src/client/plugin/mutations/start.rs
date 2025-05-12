#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-plugin-schema.graphql"#, module = "crate::schema_plugin::schema")]
pub mod mutations {
    use crate::Plugin;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariables;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PluginByNameVariables")]
    pub struct StartPlugin {
        #[arguments(name: $name)]
        pub start: Plugin,
    }

    pub fn start(name: String) -> cynic::Operation<StartPlugin, PluginByNameVariables> {
        use cynic::MutationBuilder;
        StartPlugin::build(name.into())
    }
}
