#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::extension::ExtensionDefinitions;
    use crate::schema_graphql::types::flow_type::FlowType;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinitions;
    use reactive_graph_graph::NamespacedTypeGetter;

    use crate::client::types::flows::variables::create::variables::CreateFlowTypeVariables;
    use crate::client::types::flows::variables::create::variables::CreateFlowTypeVariablesFields;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "CreateFlowTypeVariables")]
    pub struct CreateFlowType {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateFlowTypeVariables")]
    pub struct MutationTypes {
        pub flows: MutationFlowTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateFlowTypeVariables")]
    pub struct MutationFlowTypes {
        #[arguments(type: { name: $name, namespace: $namespace }, description: $description, wrapperEntityInstance: $wrapper_entity_instance, variables: $variables, extensions: $extensions
        )]
        pub create: FlowType,
    }

    pub fn create_flow_type_mutation(flow_type: reactive_graph_graph::FlowType) -> Operation<CreateFlowType, CreateFlowTypeVariables> {
        use cynic::MutationBuilder;
        let namespace = flow_type.namespace();
        let name = flow_type.type_name();
        let description = flow_type.description;
        let wrapper_entity_instance = flow_type.wrapper_entity_instance.into();
        let variables: PropertyTypeDefinitions = flow_type.variables.into();
        let extensions: ExtensionDefinitions = flow_type.extensions.into();
        let vars = CreateFlowTypeVariables {
            namespace,
            name,
            description: Some(description),
            wrapper_entity_instance,
            variables: Some(variables.0),
            extensions: Some(extensions.0),
        };
        CreateFlowType::build(vars)
    }

    pub fn create_flow_type_with_variables(variables: CreateFlowTypeVariables) -> Operation<CreateFlowType, CreateFlowTypeVariables> {
        use cynic::MutationBuilder;
        CreateFlowType::build(variables)
    }
}
