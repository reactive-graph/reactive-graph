#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::PropertyInstanceDefinition;
    use crate::schema_graphql::scalar::UUID;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct CreateFlowInstanceFromTypeVariables {
        pub namespace: String,
        pub type_name: String,
        pub id: Option<UUID>,
        // #[builder(default)]
        // pub description: Option<String>,
        #[builder(default)]
        pub variables: Option<Vec<PropertyInstanceDefinition>>,
        #[builder(default)]
        pub properties: Option<Vec<PropertyInstanceDefinition>>,
    }
}
