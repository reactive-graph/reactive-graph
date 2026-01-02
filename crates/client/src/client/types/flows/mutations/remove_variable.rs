#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::types::properties::variables::container::variables::PropertyContainerVariables;
    use crate::client::types::properties::variables::container::variables::PropertyContainerVariablesFields;
    use crate::schema_graphql::types::flow_type::FlowType;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::FlowTypeId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PropertyContainerVariables")]
    pub struct RemoveProperty {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "PropertyContainerVariables")]
    pub struct MutationTypes {
        pub flows: MutationFlowTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "PropertyContainerVariables")]
    pub struct MutationFlowTypes {
        #[arguments(type: $_type, variableName: $property_name)]
        pub remove_variable: FlowType,
    }

    pub fn remove_variable_mutation<FT: Into<FlowTypeId>>(ty: FT, variable_name: String) -> Operation<RemoveProperty, PropertyContainerVariables> {
        use cynic::MutationBuilder;
        RemoveProperty::build(PropertyContainerVariables::new(ty.into(), variable_name))
    }
}
