#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::common::variables::type_id::variables::TypeIdVariables;
    use crate::client::types::common::variables::type_id::variables::TypeIdVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "TypeIdVariables")]
    pub struct DeleteFlowType {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "TypeIdVariables")]
    pub struct MutationTypes {
        pub flows: MutationFlowTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "TypeIdVariables")]
    pub struct MutationFlowTypes {
        #[arguments(type: $_type)]
        pub delete: bool,
    }

    pub fn delete_flow_type_mutation<C: Into<reactive_graph_graph::FlowTypeId>>(ty: C) -> Operation<DeleteFlowType, TypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteFlowType::build(ty.into().into())
    }
}
