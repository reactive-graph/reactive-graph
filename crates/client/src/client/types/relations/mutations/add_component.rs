#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::relation_type::RelationType;
    use crate::types::components::variables::container::variables::ComponentContainerVariables;
    use crate::types::components::variables::container::variables::ComponentContainerVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "ComponentContainerVariables")]
    pub struct AddComponent {
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
        pub add_component: RelationType,
    }

    pub fn add_component_mutation(ty: reactive_graph_graph::RelationComponentTypeId) -> Operation<AddComponent, ComponentContainerVariables> {
        use cynic::MutationBuilder;
        AddComponent::build(ty.into())
    }

    pub fn add_component_with_variables(variables: ComponentContainerVariables) -> Operation<AddComponent, ComponentContainerVariables> {
        use cynic::MutationBuilder;
        AddComponent::build(variables)
    }
}
