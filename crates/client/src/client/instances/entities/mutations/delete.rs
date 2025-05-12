#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::instances::variables::uuid::queries::UuidVariables;
    use crate::client::instances::variables::uuid::queries::UuidVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;
    use uuid::Uuid;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "UuidVariables")]
    pub struct DeleteEntityInstance {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "UuidVariables")]
    pub struct MutationInstances {
        pub entities: MutationEntityInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "UuidVariables")]
    pub struct MutationEntityInstances {
        #[arguments(id: $id)]
        pub delete: bool,
    }

    pub fn delete_entity_instance_mutation(id: Uuid) -> Operation<DeleteEntityInstance, UuidVariables> {
        use cynic::MutationBuilder;
        DeleteEntityInstance::build(id.into())
    }
}
