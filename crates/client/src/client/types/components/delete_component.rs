#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::components::common::queries::ComponentTypeIdVariables;
    use crate::client::types::components::common::queries::ComponentTypeIdVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "ComponentTypeIdVariables")]
    pub struct DeleteComponent {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ComponentTypeIdVariables")]
    pub struct MutationTypes {
        pub components: MutationComponents,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ComponentTypeIdVariables")]
    pub struct MutationComponents {
        #[arguments(type: { name: $name, namespace: $namespace })]
        pub delete: bool,
    }

    pub fn delete_component_mutation<C: Into<reactive_graph_graph::ComponentTypeId>>(ty: C) -> Operation<DeleteComponent, ComponentTypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteComponent::build(ty.into().into())
    }

    pub fn delete_component_with_variables(variables: ComponentTypeIdVariables) -> Operation<DeleteComponent, ComponentTypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteComponent::build(variables)
    }
}
