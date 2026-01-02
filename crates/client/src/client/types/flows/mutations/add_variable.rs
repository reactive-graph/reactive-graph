#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::types::flows::variables::add_variable::variables::AddVariableVariables;
    use crate::client::types::flows::variables::add_variable::variables::AddVariableVariablesFields;
    use crate::schema_graphql::types::flow_type::FlowType;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::FlowTypeId;
    use reactive_graph_graph::PropertyType;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "AddVariableVariables")]
    pub struct AddVariable {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddVariableVariables")]
    pub struct MutationTypes {
        pub flows: MutationFlowTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddVariableVariables")]
    pub struct MutationFlowTypes {
        #[arguments(type: $_type, variable: $variable)]
        pub add_variable: FlowType,
    }

    pub fn add_variable_mutation<FT: Into<FlowTypeId>, PT: Into<PropertyType>>(ty: FT, property_type: PT) -> Operation<AddVariable, AddVariableVariables> {
        use cynic::MutationBuilder;
        AddVariable::build(AddVariableVariables::new(ty, property_type))
    }
}
