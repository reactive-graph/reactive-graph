#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::FlowTypeId;
    use uuid::Uuid;

    use crate::client::types::flows::variables::remove_entity_instance::variables::RemoveEntityInstanceVariables;
    use crate::client::types::flows::variables::remove_entity_instance::variables::RemoveEntityInstanceVariablesFields;
    use crate::schema_graphql::types::flow_type::FlowType;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "RemoveEntityInstanceVariables")]
    pub struct RemoveEntityInstance {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RemoveEntityInstanceVariables")]
    pub struct MutationTypes {
        pub flows: MutationFlowTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RemoveEntityInstanceVariables")]
    pub struct MutationFlowTypes {
        #[arguments(type: $_type, id: $id)]
        pub remove_entity_instance: FlowType,
    }

    pub fn remove_entity_instance_mutation<FT: Into<FlowTypeId>, ID: Into<Uuid>>(
        ty: FT,
        id: ID,
    ) -> Operation<RemoveEntityInstance, RemoveEntityInstanceVariables> {
        use cynic::MutationBuilder;
        RemoveEntityInstance::build(RemoveEntityInstanceVariables::new(ty, id))
    }
}
