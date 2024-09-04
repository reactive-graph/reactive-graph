#[cynic::schema_for_derives(file = r#"schema_plugin.graphql"#, module = "crate::schema_plugin::schema")]
pub mod mutations {
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariables;
    use crate::client::plugin::variables::by_name::variables::PluginByNameVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PluginByNameVariables")]
    pub struct UninstallPlugin {
        #[arguments(name: $name)]
        pub uninstall: bool,
    }

    pub fn uninstall(name: String) -> cynic::Operation<UninstallPlugin, PluginByNameVariables> {
        use cynic::MutationBuilder;
        UninstallPlugin::build(name.into())
    }
}
