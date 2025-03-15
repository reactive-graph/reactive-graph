#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::flows::type_id::queries::FlowTypeIdVariables;
    use crate::client::types::flows::type_id::queries::FlowTypeIdVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "FlowTypeIdVariables")]
    pub struct DeleteFlowType {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "FlowTypeIdVariables")]
    pub struct MutationTypes {
        pub flows: MutationFlowTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "FlowTypeIdVariables")]
    pub struct MutationFlowTypes {
        #[arguments(type: { name: $name, namespace: $namespace })]
        pub delete: bool,
    }

    pub fn delete_flow_type_mutation<C: Into<reactive_graph_graph::FlowTypeId>>(ty: C) -> Operation<DeleteFlowType, FlowTypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteFlowType::build(ty.into().into())
    }

    pub fn delete_flow_type_with_variables(variables: FlowTypeIdVariables) -> Operation<DeleteFlowType, FlowTypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteFlowType::build(variables)
    }
}
