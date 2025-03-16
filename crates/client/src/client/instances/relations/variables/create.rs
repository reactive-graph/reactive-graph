#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::PropertyInstanceDefinition;
    use crate::PropertyInstanceDefinitions;
    use crate::schema_graphql::scalar::UUID;
    use cynic::QueryVariables;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::RelationInstanceId;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct CreateRelationInstanceVariables {
        /// The id of the outbound entity instance.
        pub outbound_id: UUID,
        /// The relation type id namespace.
        pub namespace: String,
        /// The relation type id type name.
        pub type_name: String,
        /// The relation type id type name.
        pub instance_id: String,
        /// The id of the inbound entity instance.
        pub inbound_id: UUID,
        /// The description of the relation instance.
        #[builder(default)]
        pub description: Option<String>,
        /// The properties of the relation instance.
        #[builder(default)]
        pub properties: Option<Vec<PropertyInstanceDefinition>>,
    }

    impl CreateRelationInstanceVariables {
        pub fn new(id: &RelationInstanceId, description: Option<String>, properties: reactive_graph_graph::PropertyInstances) -> Self {
            let ty = id.ty.relation_type_id();
            let properties: PropertyInstanceDefinitions = properties.into();
            let properties = Some(properties.0);
            Self {
                outbound_id: id.outbound_id.into(),
                namespace: ty.namespace(),
                type_name: ty.type_name(),
                instance_id: id.ty.instance_id(),
                inbound_id: id.inbound_id.into(),
                description,
                properties,
            }
        }
    }
}
