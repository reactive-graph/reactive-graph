#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::schema_graphql::scalar::UUID;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinition;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinitions;
    use cynic::QueryVariables;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::PropertyTypes;
    use reactive_graph_graph::RelationInstanceId;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct AddPropertiesVariables {
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
        pub properties: Option<Vec<PropertyTypeDefinition>>,
    }

    impl AddPropertiesVariables {
        pub fn new(id: &RelationInstanceId, property_types: PropertyTypes) -> Self {
            let ty = id.ty.relation_type_id();
            let property_types: PropertyTypeDefinitions = property_types.into();
            Self {
                outbound_id: id.outbound_id.into(),
                namespace: ty.namespace(),
                name: ty.type_name(),
                instance_id: id.ty.instance_id(),
                inbound_id: id.inbound_id.into(),
                properties: Some(property_types.0),
            }
        }
    }
}
