#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::schema_graphql::types::component::Component;
    use crate::types::common::variables::type_id::variables::TypeIdVariables;
    use crate::types::common::variables::type_id::variables::TypeIdVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::RelationTypeId;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "TypeIdVariables")]
    pub struct GetRelationTypeComponents {
        pub types: GetRelationTypeComponentsTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Types", variables = "TypeIdVariables")]
    pub struct GetRelationTypeComponentsTypes {
        #[arguments(type: $_type)]
        pub relations: Vec<GetRelationTypeComponentsComponents>,
    }

    #[derive(QueryFragment, Debug, Clone)]
    #[cynic(graphql_type = "RelationType", variables = "TypeIdVariables")]
    pub struct GetRelationTypeComponentsComponents {
        pub components: Vec<Component>,
    }

    pub fn get_relation_type_components_query(ty: &RelationTypeId) -> Operation<GetRelationTypeComponents, TypeIdVariables> {
        use cynic::QueryBuilder;
        GetRelationTypeComponents::build(ty.clone().into())
    }
}
