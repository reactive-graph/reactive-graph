#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::client::instances::relations::variables::id::variables::RelationInstanceIdVariables;
    use crate::client::instances::relations::variables::id::variables::RelationInstanceIdVariablesFields;
    use crate::schema_graphql::instances::relation_instance::RelationInstance;
    use cynic::Operation;
    use cynic::QueryFragment;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "RelationInstanceIdVariables")]
    pub struct GetRelationInstanceById {
        pub instances: GetRelationInstanceByIdInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Instances", variables = "RelationInstanceIdVariables")]
    pub struct GetRelationInstanceByIdInstances {
        #[arguments(id: { outboundId: $outbound_id, type: $_type, instanceId: $instance_id, inboundId: $inbound_id } )]
        pub relations: Vec<RelationInstance>,
    }

    pub fn get_by_id(id: &reactive_graph_graph::RelationInstanceId) -> Operation<GetRelationInstanceById, RelationInstanceIdVariables> {
        use cynic::QueryBuilder;
        let id = id.into();
        GetRelationInstanceById::build(id)
    }
}
