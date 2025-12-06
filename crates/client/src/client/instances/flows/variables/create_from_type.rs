#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::PropertyInstanceDefinition;
    use crate::PropertyInstanceDefinitions;
    use crate::schema_graphql::scalar::UUID;
    use cynic::QueryVariables;
    use reactive_graph_graph::FlowTypeId;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::PropertyInstances;
    use uuid::Uuid;

    #[derive(QueryVariables, Debug)]
    pub struct CreateFlowInstanceFromTypeVariables {
        #[cynic(rename = "type")]
        pub _type: String,
        pub id: Option<UUID>,
        // pub description: Option<String>,
        pub variables: Option<Vec<PropertyInstanceDefinition>>,
        pub properties: Option<Vec<PropertyInstanceDefinition>>,
    }

    impl CreateFlowInstanceFromTypeVariables {
        pub fn new(ty: FlowTypeId, id: Option<Uuid>, variables: PropertyInstances, properties: PropertyInstances) -> Self {
            Self {
                _type: ty.namespace().to_string(),
                id: id.map(|id| id.into()),
                variables: Some(PropertyInstanceDefinitions::from(variables).0),
                properties: Some(PropertyInstanceDefinitions::from(properties).0),
            }
        }
    }
}
