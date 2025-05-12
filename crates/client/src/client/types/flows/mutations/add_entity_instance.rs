#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::flows::variables::add_entity_instance::variables::AddEntityInstanceVariables;
    use crate::client::types::flows::variables::add_entity_instance::variables::AddEntityInstanceVariablesFields;

    use crate::schema_graphql::instances::entity_instance::EntityInstanceDefinition;
    use crate::schema_graphql::types::flow_type::FlowType;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "AddEntityInstanceVariables")]
    pub struct AddEntityInstance {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddEntityInstanceVariables")]
    pub struct MutationTypes {
        pub flows: MutationFlowTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddEntityInstanceVariables")]
    pub struct MutationFlowTypes {
        #[arguments(type: { name: $name, namespace: $namespace }, entityInstance: $entity_instance)]
        pub add_entity_instance: FlowType,
    }

    pub fn add_entity_instance_mutation(
        ty: reactive_graph_graph::FlowTypeId,
        entity_instance: reactive_graph_graph::EntityInstance,
    ) -> Operation<AddEntityInstance, AddEntityInstanceVariables> {
        use cynic::MutationBuilder;
        let entity_instance: EntityInstanceDefinition = entity_instance.into();
        let vars = AddEntityInstanceVariables {
            namespace: ty.namespace(),
            name: ty.type_name(),
            entity_instance,
        };
        AddEntityInstance::build(vars)
    }

    pub fn add_entity_instance_with_variables(variables: AddEntityInstanceVariables) -> Operation<AddEntityInstance, AddEntityInstanceVariables> {
        use cynic::MutationBuilder;
        AddEntityInstance::build(variables)
    }
}
