#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::schema_graphql::types::extension::ExtensionDefinition;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinition;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct CreateComponentVariables {
        pub namespace: String,
        pub name: String,
        #[builder(default)]
        pub description: Option<String>,
        #[builder(default)]
        pub properties: Option<Vec<PropertyTypeDefinition>>,
        #[builder(default)]
        pub extensions: Option<Vec<ExtensionDefinition>>,
    }
}
