#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct ExtensionContainerVariables {
        pub namespace: String,
        pub name: String,
        pub extension_namespace: String,
        pub extension_name: String,
    }
}
