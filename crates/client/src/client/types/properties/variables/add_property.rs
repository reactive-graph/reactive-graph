#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::PropertyTypeDefinition;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct AddPropertyVariables {
        pub namespace: String,
        pub name: String,
        pub property: PropertyTypeDefinition,
    }
}
