#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::schema_graphql::instances::entity_instance::EntityInstanceDefinition;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct AddEntityInstanceVariables {
        pub namespace: String,
        pub name: String,
        pub entity_instance: EntityInstanceDefinition,
    }
}
