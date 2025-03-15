#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::instances::variables::uuid::queries::UuidVariables;
    use crate::client::instances::variables::uuid::queries::UuidVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;
    use uuid::Uuid;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "UuidVariables")]
    pub struct DeleteFlowInstance {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "UuidVariables")]
    pub struct MutationInstances {
        pub flows: MutationFlowInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "UuidVariables")]
    pub struct MutationFlowInstances {
        #[arguments(id: $id)]
        pub delete: bool,
    }

    pub fn delete_flow_instance_mutation(id: Uuid) -> Operation<DeleteFlowInstance, UuidVariables> {
        use cynic::MutationBuilder;
        DeleteFlowInstance::build(id.into())
    }
}
