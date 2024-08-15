#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::schema_graphql::types::component::Component;
    use crate::types::entities::type_id::queries::EntityTypeIdVariables;
    use crate::types::entities::type_id::queries::EntityTypeIdVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::EntityTypeId;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "EntityTypeIdVariables")]
    pub struct GetEntityTypeComponents {
        pub types: GetEntityTypeComponentsTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Types", variables = "EntityTypeIdVariables")]
    pub struct GetEntityTypeComponentsTypes {
        #[arguments(
          type: {
            namespace: $namespace,
            name: $name
          }
        )]
        pub entities: Vec<GetEntityTypeComponentsComponents>,
    }

    #[derive(QueryFragment, Debug, Clone)]
    #[cynic(graphql_type = "EntityType", variables = "EntityTypeIdVariables")]
    pub struct GetEntityTypeComponentsComponents {
        pub components: Vec<Component>,
    }

    pub fn get_entity_type_components_query(ty: &EntityTypeId) -> Operation<GetEntityTypeComponents, EntityTypeIdVariables> {
        use cynic::QueryBuilder;
        GetEntityTypeComponents::build(ty.clone().into())
    }
}
