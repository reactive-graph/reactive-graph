#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::schema_graphql::scalar::UUID;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct RemoveEntityInstanceVariables {
        pub namespace: String,
        pub name: String,
        pub id: UUID,
    }
}
