#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::schema_graphql::types::component::Component;
    use crate::types::relations::type_id::queries::RelationTypeIdVariables;
    use crate::types::relations::type_id::queries::RelationTypeIdVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::RelationTypeId;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "RelationTypeIdVariables")]
    pub struct GetRelationTypeComponents {
        pub types: GetRelationTypeComponentsTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Types", variables = "RelationTypeIdVariables")]
    pub struct GetRelationTypeComponentsTypes {
        #[arguments(
          type: {
            namespace: $namespace,
            name: $name
          }
        )]
        pub relations: Vec<GetRelationTypeComponentsComponents>,
    }

    #[derive(QueryFragment, Debug, Clone)]
    #[cynic(graphql_type = "RelationType", variables = "RelationTypeIdVariables")]
    pub struct GetRelationTypeComponentsComponents {
        pub components: Vec<Component>,
    }

    pub fn get_relation_type_components_query(ty: &RelationTypeId) -> Operation<GetRelationTypeComponents, RelationTypeIdVariables> {
        use cynic::QueryBuilder;
        GetRelationTypeComponents::build(ty.clone().into())
    }
}
