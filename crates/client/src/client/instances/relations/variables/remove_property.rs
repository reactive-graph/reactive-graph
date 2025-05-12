#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::schema_graphql::scalar::UUID;
    use cynic::QueryVariables;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::PropertyType;
    use reactive_graph_graph::PropertyTypes;
    use reactive_graph_graph::RelationInstanceId;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct RemovePropertiesVariables {
        /// The id of the outbound entity instance.
        pub outbound_id: UUID,
        /// The relation type id namespace.
        pub namespace: String,
        /// The relation type id type name.
        pub name: String,
        /// The instance id of the relation_instance_type_id.
        pub instance_id: String,
        /// The id of the inbound entity instance.
        pub inbound_id: UUID,
        /// The properties to add to the relation instance.
        pub properties: Option<Vec<String>>,
    }

    impl RemovePropertiesVariables {
        pub fn new(id: &RelationInstanceId, property_names: Vec<String>) -> Self {
            let ty = id.ty.relation_type_id();
            Self {
                outbound_id: id.outbound_id.into(),
                namespace: ty.namespace(),
                name: ty.type_name(),
                instance_id: id.ty.instance_id(),
                inbound_id: id.inbound_id.into(),
                properties: Some(property_names),
            }
        }

        pub fn new_from_property_name(id: &RelationInstanceId, property_name: String) -> Self {
            Self::new(id, vec![property_name])
        }

        pub fn new_from_property_types<P: Into<PropertyTypes>>(id: &RelationInstanceId, property_types: P) -> Self {
            let property_types = property_types.into();
            Self::new(id, property_types.names())
        }
        pub fn new_from_property_type<P: Into<PropertyType>>(id: &RelationInstanceId, property_type: P) -> Self {
            let property_type = property_type.into();
            Self::new(id, vec![property_type.name.clone()])
        }
    }
}
