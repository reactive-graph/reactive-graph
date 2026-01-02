#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::schema_graphql::instances::property_instance::PropertyInstanceDefinition;
    use crate::schema_graphql::scalar::id::UUID;
    use cynic::QueryVariables;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::RelationInstanceId;
    use serde_json::Value;

    #[derive(QueryVariables, Debug)]
    pub struct SetPropertyVariables {
        /// The id of the outbound entity instance.
        pub outbound_id: UUID,
        /// The fully qualified namespace of the relation type.
        #[cynic(rename = "type")]
        pub _type: String,
        /// The instance id of the relation_instance_type_id.
        pub instance_id: String,
        /// The id of the inbound entity instance.
        pub inbound_id: UUID,
        /// The properties to set.
        pub properties: Option<Vec<PropertyInstanceDefinition>>,
    }

    impl SetPropertyVariables {
        pub fn new(id: &RelationInstanceId, name: String, value: Value) -> Self {
            let ty = id.ty.relation_type_id();
            Self {
                outbound_id: id.outbound_id.into(),
                _type: ty.namespace().to_string(),
                instance_id: id.ty.instance_id().to_string(),
                inbound_id: id.inbound_id.into(),
                properties: Some(vec![PropertyInstanceDefinition { name, value }]),
            }
        }
    }
}
