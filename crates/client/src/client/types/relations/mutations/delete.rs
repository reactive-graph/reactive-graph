#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::common::variables::type_id::variables::TypeIdVariables;
    use crate::client::types::common::variables::type_id::variables::TypeIdVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "TypeIdVariables")]
    pub struct DeleteRelationType {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "TypeIdVariables")]
    pub struct MutationTypes {
        pub relations: MutationRelationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "TypeIdVariables")]
    pub struct MutationRelationTypes {
        #[arguments(type: { name: $name, namespace: $namespace })]
        pub delete: bool,
    }

    pub fn delete_relation_type_mutation<C: Into<reactive_graph_graph::RelationTypeId>>(ty: C) -> Operation<DeleteRelationType, TypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteRelationType::build(ty.into().into())
    }

    pub fn delete_relation_type_with_variables(variables: TypeIdVariables) -> Operation<DeleteRelationType, TypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteRelationType::build(variables)
    }
}
