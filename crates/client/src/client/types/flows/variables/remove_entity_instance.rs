#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;
    use reactive_graph_graph::FlowTypeId;
    use reactive_graph_graph::NamespacedTypeGetter;
    use uuid::Uuid;

    use crate::schema_graphql::scalar::UUID;

    #[derive(QueryVariables, Debug)]
    pub struct RemoveEntityInstanceVariables {
        #[cynic(rename = "type")]
        pub _type: String,
        pub id: UUID,
    }

    impl RemoveEntityInstanceVariables {
        pub fn new<FT: Into<FlowTypeId>, ID: Into<Uuid>>(ty: FT, id: ID) -> Self {
            Self {
                _type: ty.into().namespace().to_string(),
                id: id.into().into(),
            }
        }
    }
}
