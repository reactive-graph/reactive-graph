#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::relation_type::RelationType;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct GetAllRelationTypes {
        pub types: GetAllRelationTypesTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Types")]
    pub struct GetAllRelationTypesTypes {
        pub relations: Vec<RelationType>,
    }

    pub fn get_all_relation_types_query() -> Operation<GetAllRelationTypes, ()> {
        use cynic::QueryBuilder;
        GetAllRelationTypes::build(())
    }
}
