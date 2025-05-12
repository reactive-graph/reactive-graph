#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::common::variables::type_id::variables::TypeIdVariables;
    use crate::client::types::common::variables::type_id::variables::TypeIdVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "TypeIdVariables")]
    pub struct DeleteEntityType {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "TypeIdVariables")]
    pub struct MutationTypes {
        pub entities: MutationEntityTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "TypeIdVariables")]
    pub struct MutationEntityTypes {
        #[arguments(type: { name: $name, namespace: $namespace })]
        pub delete: bool,
    }

    pub fn delete_entity_type_mutation<C: Into<reactive_graph_graph::EntityTypeId>>(ty: C) -> Operation<DeleteEntityType, TypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteEntityType::build(ty.into().into())
    }

    pub fn delete_entity_type_with_variables(variables: TypeIdVariables) -> Operation<DeleteEntityType, TypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteEntityType::build(variables)
    }
}
