#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::component::Component;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct GetAllComponents {
        pub types: GetAllComponentsTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Types")]
    pub struct GetAllComponentsTypes {
        pub components: Vec<Component>,
    }

    pub fn get_all_components_query() -> Operation<GetAllComponents, ()> {
        use cynic::QueryBuilder;
        GetAllComponents::build(())
    }
}
