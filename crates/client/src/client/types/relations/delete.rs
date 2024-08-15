#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::relations::type_id::queries::RelationTypeIdVariables;
    use crate::client::types::relations::type_id::queries::RelationTypeIdVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "RelationTypeIdVariables")]
    pub struct DeleteRelationType {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RelationTypeIdVariables")]
    pub struct MutationTypes {
        pub relations: MutationRelationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RelationTypeIdVariables")]
    pub struct MutationRelationTypes {
        #[arguments(type: { name: $name, namespace: $namespace })]
        pub delete: bool,
    }

    pub fn delete_relation_type_mutation<C: Into<reactive_graph_graph::RelationTypeId>>(ty: C) -> Operation<DeleteRelationType, RelationTypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteRelationType::build(ty.into().into())
    }

    pub fn delete_relation_type_with_variables(variables: RelationTypeIdVariables) -> Operation<DeleteRelationType, RelationTypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteRelationType::build(variables)
    }
}
