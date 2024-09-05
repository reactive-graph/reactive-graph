#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::schema_graphql::scalar::id::UUID;
    use cynic::QueryVariables;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::RelationInstanceId;

    #[derive(QueryVariables, Debug)]
    pub struct RelationInstanceIdVariables {
        /// The id of the outbound entity instance.
        pub outbound_id: UUID,
        /// The relation type id namespace.
        pub namespace: String,
        /// The relation type id type name.
        pub name: String,
        /// The id of the inbound entity instance.
        pub inbound_id: UUID,
    }

    impl From<&RelationInstanceId> for RelationInstanceIdVariables {
        fn from(relation_instance_id: &RelationInstanceId) -> Self {
            Self {
                outbound_id: relation_instance_id.outbound_id.into(),
                namespace: relation_instance_id.namespace(),
                name: relation_instance_id.type_name(),
                inbound_id: relation_instance_id.inbound_id.into(),
            }
        }
    }
}
