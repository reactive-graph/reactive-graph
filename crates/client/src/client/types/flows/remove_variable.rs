#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::properties::container::queries::PropertyContainerVariables;
    use crate::client::types::properties::container::queries::PropertyContainerVariablesFields;
    use crate::schema_graphql::types::flow_type::FlowType;
    use reactive_graph_graph::NamespacedTypeGetter;

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
        #[arguments(type: { name: $name, namespace: $namespace }, variableName: $property_name)]
        pub remove_variable: FlowType,
    }

    pub fn remove_variable_mutation(ty: reactive_graph_graph::FlowTypeId, variable_name: String) -> Operation<RemoveProperty, PropertyContainerVariables> {
        use cynic::MutationBuilder;
        let vars = PropertyContainerVariables {
            namespace: ty.namespace(),
            name: ty.type_name(),
            property_name: variable_name,
        };
        RemoveProperty::build(vars)
    }

    pub fn remove_variable_with_variables(variables: PropertyContainerVariables) -> Operation<RemoveProperty, PropertyContainerVariables> {
        use cynic::MutationBuilder;
        RemoveProperty::build(variables)
    }
}
