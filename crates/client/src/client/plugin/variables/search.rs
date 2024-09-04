#[cynic::schema_for_derives(file = r#"schema_runtime.graphql"#, module = "crate::schema_runtime::schema")]
pub mod variables {
    use typed_builder::TypedBuilder;
    #[derive(cynic::QueryVariables, Debug, TypedBuilder)]
    pub struct SearchPluginVariables {
        pub name: Option<String>,
        pub state: Option<String>,
        pub stem: Option<String>,
    }
}
