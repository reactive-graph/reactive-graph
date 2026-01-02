#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
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
