#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::relation_type::RelationType;
    use reactive_graph_graph::RelationTypeId;

    use crate::types::common::variables::type_id::variables::TypeIdVariables;
    use crate::types::common::variables::type_id::variables::TypeIdVariablesFields;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "TypeIdVariables")]
    pub struct GetRelationTypeByType {
        pub types: GetRelationTypeByTypeTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Types", variables = "TypeIdVariables")]
    pub struct GetRelationTypeByTypeTypes {
        #[arguments(
          type: {
            namespace: $namespace,
            name: $name
          }
        )]
        pub relations: Vec<RelationType>,
    }

    pub fn get_relation_type_by_type_query(ty: &RelationTypeId) -> Operation<GetRelationTypeByType, TypeIdVariables> {
        use cynic::QueryBuilder;
        GetRelationTypeByType::build(ty.clone().into())
    }
}
