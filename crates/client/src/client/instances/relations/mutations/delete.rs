#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::instances::relations::variables::id::variables::RelationInstanceIdVariables;
    use crate::client::instances::relations::variables::id::variables::RelationInstanceIdVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::RelationInstanceId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "RelationInstanceIdVariables")]
    pub struct DeleteRelationInstance {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RelationInstanceIdVariables")]
    pub struct MutationInstances {
        pub relations: MutationRelationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RelationInstanceIdVariables")]
    pub struct MutationRelationInstances {
        #[arguments(relationInstanceId: { outboundId: $outbound_id, namespace: $namespace, typeName: $name, instanceId: $instance_id, inboundId: $inbound_id})]
        pub delete: bool,
    }

    pub fn delete(id: &RelationInstanceId) -> Operation<DeleteRelationInstance, RelationInstanceIdVariables> {
        use cynic::MutationBuilder;
        DeleteRelationInstance::build(id.into())
    }
}
