#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-runtime-schema.graphql"#, module = "crate::schema_runtime::schema")]
pub mod variables {
    #[derive(cynic::QueryVariables, Debug)]
    pub struct PluginByNameVariables {
        pub name: String,
    }

    impl From<String> for PluginByNameVariables {
        fn from(name: String) -> Self {
            PluginByNameVariables { name }
        }
    }
}
