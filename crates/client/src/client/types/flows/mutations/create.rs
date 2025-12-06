#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::flows::variables::create::variables::CreateFlowTypeVariables;
    use crate::client::types::flows::variables::create::variables::CreateFlowTypeVariablesFields;
    use crate::schema_graphql::types::flow_type::FlowType;

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
        #[arguments(type: $_type, description: $description, wrapperEntityInstance: $wrapper_entity_instance, variables: $variables, extensions: $extensions
        )]
        pub create: FlowType,
    }

    pub fn create_flow_type_mutation(flow_type: reactive_graph_graph::FlowType) -> Operation<CreateFlowType, CreateFlowTypeVariables> {
        use cynic::MutationBuilder;
        CreateFlowType::build(CreateFlowTypeVariables::new(flow_type))
    }
}
