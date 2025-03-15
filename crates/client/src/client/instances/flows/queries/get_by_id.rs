#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::client::instances::variables::uuid::queries::UuidVariables;
    use crate::client::instances::variables::uuid::queries::UuidVariablesFields;
    use crate::schema_graphql::instances::flow_instance::FlowInstance;
    use cynic::Operation;
    use cynic::QueryFragment;
    use uuid::Uuid;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "UuidVariables")]
    pub struct GetFlowInstanceById {
        pub instances: GetFlowInstanceByIdInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Instances", variables = "UuidVariables")]
    pub struct GetFlowInstanceByIdInstances {
        #[arguments(id: $id)]
        pub flows: Vec<FlowInstance>,
    }

    pub fn get_flow_instance_by_id(id: Uuid) -> Operation<GetFlowInstanceById, UuidVariables> {
        use cynic::QueryBuilder;
        GetFlowInstanceById::build(id.into())
    }
}
