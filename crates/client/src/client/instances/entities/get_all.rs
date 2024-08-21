#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::instances::entity_instance::EntityInstance;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct GetAllEntityInstances {
        pub instances: GetAllEntityInstancesInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Instances")]
    pub struct GetAllEntityInstancesInstances {
        pub entities: Vec<EntityInstance>,
    }

    pub fn get_all_entity_instances_query() -> Operation<GetAllEntityInstances, ()> {
        use cynic::QueryBuilder;
        GetAllEntityInstances::build(())
    }
}
