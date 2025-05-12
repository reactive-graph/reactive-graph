#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::schema_graphql::scalar::UUID;
    use crate::schema_graphql::types::component::ComponentTypeId;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct IdAndComponentVariables {
        pub id: UUID,
        pub components: Option<Vec<ComponentTypeId>>,
    }
}
