#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::entity_types::type_id::queries::EntityTypeIdVariables;
    use crate::client::types::entity_types::type_id::queries::EntityTypeIdVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "EntityTypeIdVariables")]
    pub struct DeleteEntityType {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "EntityTypeIdVariables")]
    pub struct MutationTypes {
        pub entities: MutationEntityTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "EntityTypeIdVariables")]
    pub struct MutationEntityTypes {
        #[arguments(type: { name: $name, namespace: $namespace })]
        pub delete: bool,
    }

    pub fn delete_entity_type_mutation<C: Into<reactive_graph_graph::EntityTypeId>>(ty: C) -> Operation<DeleteEntityType, EntityTypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteEntityType::build(ty.into().into())
    }

    pub fn delete_entity_type_with_variables(variables: EntityTypeIdVariables) -> Operation<DeleteEntityType, EntityTypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteEntityType::build(variables)
    }
}
