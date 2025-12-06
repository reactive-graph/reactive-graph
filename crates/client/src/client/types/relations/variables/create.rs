#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::ExtensionDefinition;
    use crate::PropertyTypeDefinition;
    use cynic::QueryVariables;

    #[derive(QueryVariables, Debug)]
    pub struct CreateRelationTypeVariables {
        pub outbound_entity_type: Option<String>,
        pub outbound_component: Option<String>,
        pub _type: String,
        pub inbound_entity_type: Option<String>,
        pub inbound_component: Option<String>,
        pub description: Option<String>,
        pub properties: Option<Vec<PropertyTypeDefinition>>,
        pub extensions: Option<Vec<ExtensionDefinition>>,
    }
}
