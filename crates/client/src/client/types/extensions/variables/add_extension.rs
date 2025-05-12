#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use crate::ExtensionDefinition;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct AddExtensionVariables {
        pub namespace: String,
        pub name: String,
        pub extension: ExtensionDefinition,
    }
}
