#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::client::instances::variables::uuid::queries::UuidVariables;
    use crate::client::instances::variables::uuid::queries::UuidVariablesFields;
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use cynic::Operation;
    use cynic::QueryFragment;
    use uuid::Uuid;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "UuidVariables")]
    pub struct GetEntityInstanceById {
        pub instances: GetEntityInstanceByIdInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Instances", variables = "UuidVariables")]
    pub struct GetEntityInstanceByIdInstances {
        #[arguments(id: $id)]
        pub entities: Vec<EntityInstance>,
    }

    pub fn get_entity_instance_by_id(id: Uuid) -> Operation<GetEntityInstanceById, UuidVariables> {
        use cynic::QueryBuilder;
        GetEntityInstanceById::build(id.into())
    }
}
