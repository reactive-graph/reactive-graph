#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::ExtensionDefinitions;
    use crate::PropertyTypeDefinitions;
    use crate::schema_graphql::types::extension::ExtensionDefinition;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinition;
    use cynic::QueryVariables;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug)]
    pub struct CreateComponentVariables {
        pub _type: String,
        pub description: Option<String>,
        pub properties: Option<Vec<PropertyTypeDefinition>>,
        pub extensions: Option<Vec<ExtensionDefinition>>,
    }

    impl CreateComponentVariables {
        pub fn new(component: reactive_graph_graph::Component) -> Self {
            Self {
                _type: component.namespace().to_string(),
                description: Some(component.description),
                properties: Some(PropertyTypeDefinitions::from(component.properties).0),
                extensions: Some(ExtensionDefinitions::from(component.extensions).0),
            }
        }
    }
}
