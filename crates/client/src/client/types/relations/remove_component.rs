#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::relation_type::RelationType;
    use crate::types::components::container::queries::ComponentContainerVariables;
    use crate::types::components::container::queries::ComponentContainerVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "ComponentContainerVariables")]
    pub struct RemoveComponent {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ComponentContainerVariables")]
    pub struct MutationTypes {
        pub relations: MutationRelationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ComponentContainerVariables")]
    pub struct MutationRelationTypes {
        #[arguments(type: { name: $name, namespace: $namespace }, component: { name: $component_name, namespace: $component_namespace }
        )]
        pub remove_component: RelationType,
    }

    pub fn remove_component_mutation(ty: reactive_graph_graph::RelationComponentTypeId) -> Operation<RemoveComponent, ComponentContainerVariables> {
        use cynic::MutationBuilder;
        RemoveComponent::build(ty.into())
    }

    pub fn remove_component_with_variables(variables: ComponentContainerVariables) -> Operation<RemoveComponent, ComponentContainerVariables> {
        use cynic::MutationBuilder;
        RemoveComponent::build(variables)
    }
}
