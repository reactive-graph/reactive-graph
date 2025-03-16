#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::PropertyInstanceDefinitions;
    use crate::client::instances::flows::variables::create_from_type::variables::CreateFlowInstanceFromTypeVariables;
    use crate::client::instances::flows::variables::create_from_type::variables::CreateFlowInstanceFromTypeVariablesFields;
    use crate::schema_graphql::instances::flow_instance::FlowInstance;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::FlowTypeId;
    use reactive_graph_graph::NamespacedTypeGetter;
    use uuid::Uuid;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "CreateFlowInstanceFromTypeVariables")]
    pub struct CreateFlowInstance {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateFlowInstanceFromTypeVariables")]
    pub struct MutationInstances {
        pub flows: MutationFlowInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateFlowInstanceFromTypeVariables")]
    pub struct MutationFlowInstances {
        #[arguments(type: { namespace: $namespace, name: $type_name}, variables: $variables, properties: $properties
        )]
        pub create_from_type: FlowInstance,
    }

    pub fn create_flow_instance_from_type_mutation(
        ty: FlowTypeId,
        id: Option<Uuid>,
        variables: reactive_graph_graph::PropertyInstances,
        properties: reactive_graph_graph::PropertyInstances,
    ) -> Operation<CreateFlowInstance, CreateFlowInstanceFromTypeVariables> {
        use cynic::MutationBuilder;
        let id = id.map(|id| id.into());
        let variables: PropertyInstanceDefinitions = variables.into();
        let variables = Some(variables.0);
        let properties: PropertyInstanceDefinitions = properties.into();
        let properties = Some(properties.0);
        let vars = CreateFlowInstanceFromTypeVariables::builder()
            .namespace(ty.namespace())
            .type_name(ty.type_name())
            .id(id)
            .variables(variables)
            .properties(properties)
            .build();
        CreateFlowInstance::build(vars)
    }
}
