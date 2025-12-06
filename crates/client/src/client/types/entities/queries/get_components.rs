#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::schema_graphql::types::component::Component;
    use crate::types::common::variables::type_id::variables::TypeIdVariables;
    use crate::types::common::variables::type_id::variables::TypeIdVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::EntityTypeId;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "TypeIdVariables")]
    pub struct GetEntityTypeComponents {
        pub types: GetEntityTypeComponentsTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Types", variables = "TypeIdVariables")]
    pub struct GetEntityTypeComponentsTypes {
        #[arguments(type: $_type)]
        pub entities: Vec<GetEntityTypeComponentsComponents>,
    }

    #[derive(QueryFragment, Debug, Clone)]
    #[cynic(graphql_type = "EntityType", variables = "TypeIdVariables")]
    pub struct GetEntityTypeComponentsComponents {
        pub components: Vec<Component>,
    }

    pub fn get_entity_type_components_query<E: Into<EntityTypeId>>(ty: E) -> Operation<GetEntityTypeComponents, TypeIdVariables> {
        use cynic::QueryBuilder;
        GetEntityTypeComponents::build(ty.into().into())
    }
}
