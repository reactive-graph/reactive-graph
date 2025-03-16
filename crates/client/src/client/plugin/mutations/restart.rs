#[cynic::schema_for_derives(file = r#"schema_plugin.graphql"#, module = "crate::schema_plugin::schema")]
pub mod mutations {
    use crate::Plugin;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariables;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PluginByNameVariables")]
    pub struct RestartPlugin {
        #[arguments(name: $name)]
        pub restart: Plugin,
    }

    pub fn restart(name: String) -> cynic::Operation<RestartPlugin, PluginByNameVariables> {
        use cynic::MutationBuilder;
        RestartPlugin::build(name.into())
    }
}
