#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::schema_graphql::types::entity_type::EntityType;
    use crate::types::components::variables::container::variables::ComponentContainerVariables;
    use crate::types::components::variables::container::variables::ComponentContainerVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::EntityComponentTypeId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "ComponentContainerVariables")]
    pub struct AddComponent {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ComponentContainerVariables")]
    pub struct MutationTypes {
        pub entities: MutationEntityTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ComponentContainerVariables")]
    pub struct MutationEntityTypes {
        #[arguments(type: $_type, component: $component_type)]
        pub add_component: EntityType,
    }

    pub fn add_component_mutation<EC: Into<EntityComponentTypeId>>(ty: EC) -> Operation<AddComponent, ComponentContainerVariables> {
        use cynic::MutationBuilder;
        AddComponent::build(ty.into().into())
    }
}
