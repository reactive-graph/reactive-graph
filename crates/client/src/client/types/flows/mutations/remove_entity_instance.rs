#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::schema_graphql::types::flow_type::FlowType;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::FlowTypeId;
    use reactive_graph_graph::NamespacedTypeGetter;
    use uuid::Uuid;

    use crate::client::types::flows::variables::remove_entity_instance::variables::RemoveEntityInstanceVariables;
    use crate::client::types::flows::variables::remove_entity_instance::variables::RemoveEntityInstanceVariablesFields;

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
        #[arguments(type: { name: $name, namespace: $namespace }, id: $id)]
        pub remove_entity_instance: FlowType,
    }

    pub fn remove_entity_instance_mutation<ID: Into<Uuid>>(ty: FlowTypeId, id: ID) -> Operation<RemoveEntityInstance, RemoveEntityInstanceVariables> {
        use cynic::MutationBuilder;
        let id = id.into();
        let vars = RemoveEntityInstanceVariables {
            namespace: ty.namespace(),
            name: ty.type_name(),
            id: id.into(),
        };
        RemoveEntityInstance::build(vars)
    }

    pub fn remove_entity_instance_with_variables(variables: RemoveEntityInstanceVariables) -> Operation<RemoveEntityInstance, RemoveEntityInstanceVariables> {
        use cynic::MutationBuilder;
        RemoveEntityInstance::build(variables)
    }
}
