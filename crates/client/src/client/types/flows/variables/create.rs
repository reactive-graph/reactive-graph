#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::ExtensionDefinition;
    use crate::ExtensionDefinitions;
    use crate::PropertyTypeDefinition;
    use crate::PropertyTypeDefinitions;
    use crate::schema_graphql::instances::entity_instance::EntityInstanceDefinition;
    use cynic::QueryVariables;
    use reactive_graph_graph::FlowType;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug)]
    pub struct CreateFlowTypeVariables {
        pub _type: String,
        pub description: Option<String>,
        pub wrapper_entity_instance: EntityInstanceDefinition,
        pub variables: Option<Vec<PropertyTypeDefinition>>,
        pub extensions: Option<Vec<ExtensionDefinition>>,
    }

    impl CreateFlowTypeVariables {
        pub fn new(flow_type: FlowType) -> Self {
            Self {
                _type: flow_type.namespace().to_string(),
                description: Some(flow_type.description),
                wrapper_entity_instance: flow_type.wrapper_entity_instance.into(),
                variables: Some(PropertyTypeDefinitions::from(flow_type.variables).0),
                extensions: Some(ExtensionDefinitions::from(flow_type.extensions).0),
            }
        }
    }
}
