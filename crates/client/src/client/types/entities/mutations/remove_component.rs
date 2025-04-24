#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::entity_type::EntityType;
    use crate::types::components::variables::container::variables::ComponentContainerVariables;
    use crate::types::components::variables::container::variables::ComponentContainerVariablesFields;

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
        #[arguments(type: { name: $name, namespace: $namespace }, component: { name: $component_name, namespace: $component_namespace }
        )]
        pub remove_component: EntityType,
    }

    pub fn remove_component_mutation(ty: reactive_graph_graph::EntityComponentTypeId) -> Operation<RemoveComponent, ComponentContainerVariables> {
        use cynic::MutationBuilder;
        RemoveComponent::build(ty.into())
    }

    pub fn remove_component_with_variables(variables: ComponentContainerVariables) -> Operation<RemoveComponent, ComponentContainerVariables> {
        use cynic::MutationBuilder;
        RemoveComponent::build(variables)
    }
}
