#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::schema_graphql::scalar::id::UUID;
    use cynic::QueryVariables;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::RelationInstanceId;

    #[derive(QueryVariables, Debug)]
    pub struct RelationInstanceIdVariables {
        /// The id of the outbound entity instance.
        pub outbound_id: UUID,
        /// The fully qualified namespace of the relation type.
        #[cynic(rename = "type")]
        pub _type: String,
        /// The relation type id type name.
        pub instance_id: String,
        /// The id of the inbound entity instance.
        pub inbound_id: UUID,
    }

    impl From<&RelationInstanceId> for RelationInstanceIdVariables {
        fn from(id: &RelationInstanceId) -> Self {
            let ty = id.ty.relation_type_id();
            Self {
                outbound_id: id.outbound_id.into(),
                _type: ty.namespace().to_string(),
                instance_id: id.ty.instance_id().to_string(),
                inbound_id: id.inbound_id.into(),
            }
        }
    }
}
