#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::types::common::variables::type_id::variables::TypeIdVariables;
    use crate::client::types::common::variables::type_id::variables::TypeIdVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::RelationTypeId;

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
        #[arguments(type: $_type)]
        pub delete: bool,
    }

    pub fn delete_relation_type_mutation<C: Into<RelationTypeId>>(ty: C) -> Operation<DeleteRelationType, TypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteRelationType::build(ty.into().into())
    }
}
