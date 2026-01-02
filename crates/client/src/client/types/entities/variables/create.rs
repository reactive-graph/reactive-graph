#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;

    use crate::ExtensionDefinition;
    use crate::PropertyTypeDefinition;

    #[derive(QueryVariables, Debug)]
    pub struct CreateEntityTypeVariables {
        pub _type: String,
        pub description: Option<String>,
        pub properties: Option<Vec<PropertyTypeDefinition>>,
        pub extensions: Option<Vec<ExtensionDefinition>>,
    }
}
