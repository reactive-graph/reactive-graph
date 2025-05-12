#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::flows::variables::add_variable::variables::AddVariableVariables;
    use crate::client::types::flows::variables::add_variable::variables::AddVariableVariablesFields;
    use crate::schema_graphql::types::extension::ExtensionDefinitions;
    use crate::schema_graphql::types::flow_type::FlowType;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinition;
    use reactive_graph_graph::NamespacedTypeGetter;

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
        #[arguments(type: { name: $name, namespace: $namespace }, variable: $variable)]
        pub add_variable: FlowType,
    }

    pub fn add_variable_mutation(
        ty: reactive_graph_graph::FlowTypeId,
        property_type: reactive_graph_graph::PropertyType,
    ) -> Operation<AddVariable, AddVariableVariables> {
        use cynic::MutationBuilder;
        let extensions: ExtensionDefinitions = property_type.extensions.into();
        let vars = AddVariableVariables {
            namespace: ty.namespace(),
            name: ty.type_name(),
            variable: PropertyTypeDefinition {
                name: property_type.name,
                data_type: property_type.data_type.into(),
                description: property_type.description,
                socket_type: property_type.socket_type.into(),
                mutability: property_type.mutability.into(),
                extensions: extensions.0,
            },
        };
        AddVariable::build(vars)
    }

    pub fn add_variable_with_variables(variables: AddVariableVariables) -> Operation<AddVariable, AddVariableVariables> {
        use cynic::MutationBuilder;
        AddVariable::build(variables)
    }
}
