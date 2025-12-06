#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::flows::variables::add_entity_instance::variables::AddEntityInstanceVariables;
    use crate::client::types::flows::variables::add_entity_instance::variables::AddEntityInstanceVariablesFields;

    use crate::schema_graphql::types::flow_type::FlowType;
    use reactive_graph_graph::EntityInstance;
    use reactive_graph_graph::FlowTypeId;

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
        #[arguments(type: $_type, entityInstance: $entity_instance)]
        pub add_entity_instance: FlowType,
    }

    pub fn add_entity_instance_mutation<FT: Into<FlowTypeId>, EI: Into<EntityInstance>>(
        ty: FT,
        entity_instance: EI,
    ) -> Operation<AddEntityInstance, AddEntityInstanceVariables> {
        use cynic::MutationBuilder;
        AddEntityInstance::build(AddEntityInstanceVariables::new(ty.into(), entity_instance.into()))
    }
}
