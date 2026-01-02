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
    pub struct RemoveComponent {
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
        pub remove_component: EntityType,
    }

    pub fn remove_component_mutation<EC: Into<EntityComponentTypeId>>(ty: EC) -> Operation<RemoveComponent, ComponentContainerVariables> {
        use cynic::MutationBuilder;
        RemoveComponent::build(ty.into().into())
    }
}
