#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::schema_graphql::scalar::UUID;
    use cynic::QueryVariables;
    use uuid::Uuid;

    #[derive(QueryVariables, Debug)]
    pub struct UuidVariables {
        pub id: UUID,
    }

    impl From<Uuid> for UuidVariables {
        fn from(id: Uuid) -> Self {
            Self { id: UUID(id) }
        }
    }
}
