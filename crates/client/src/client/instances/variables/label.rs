#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::QueryVariables;

    #[derive(QueryVariables, Debug)]
    pub struct LabelVariables {
        pub label: String,
    }

    impl From<String> for LabelVariables {
        fn from(label: String) -> Self {
            Self { label }
        }
    }
}
